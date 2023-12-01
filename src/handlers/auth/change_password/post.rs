use actix_web::{error::InternalError, web, HttpResponse};
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, validate_password_length, Credentials, UserId},
    domain::{e500, to_bad_request_response, AuthError, PasswordChangeRequest},
    queries::get_username,
};

#[tracing::instrument(
    name="Change Password"
    skip(data, pool, user_id)
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn change_password(
    data: web::Json<PasswordChangeRequest>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    validate_password_length(&data.new_password)
        .map_err(|(e, r)| InternalError::from_response(e, r))?;

    if data.new_password.expose_secret() != data.new_password_check.expose_secret() {
        let reason = "Two different passwords were entered - the field values must match.";
        let response = to_bad_request_response(reason);
        let error = anyhow::anyhow!(reason);
        return Err(InternalError::from_response(error, response).into());
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: data.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                let reason = "The current password is incorrect.";
                let response = HttpResponse::BadRequest().reason(reason).finish();
                let error = anyhow::anyhow!(reason);
                Err(InternalError::from_response(error, response).into())
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    crate::authentication::change_password(*user_id, data.0.new_password, &pool)
        .await
        .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}
