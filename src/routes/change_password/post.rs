use actix_web::{web, HttpResponse, error::InternalError};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials, UserId},
    utils::{e500, get_username},
};

#[derive(serde::Deserialize)]
pub struct PasswordChangeData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Json<PasswordChangeData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        let reason = "Two different passwords were entered - the field values must match.".into();
        let response = HttpResponse::BadRequest().reason(reason).finish();
        let error = anyhow::anyhow!(reason);
        return Err(InternalError::from_response(error, response).into())
    }

    if form.new_password.expose_secret().len() < 12 {
        let reason = "The new password is too short - it must be between 12 and 128 characters long.".into();
        let response = HttpResponse::BadRequest().reason(reason).finish();
        let error = anyhow::anyhow!(reason);
        return Err(InternalError::from_response(error, response).into())
    }

    if form.new_password.expose_secret().len() > 128 {
        let reason = "The new password is too long - it must be between 12 and 128 characters long.".into();
        let response = HttpResponse::BadRequest().reason(reason).finish();
        let error = anyhow::anyhow!(reason);
        return Err(InternalError::from_response(error, response).into())
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                let reason = "The current password is incorrect.".into();
                let response = HttpResponse::BadRequest().reason(reason).finish();
                let error = anyhow::anyhow!(reason);
                Err(InternalError::from_response(error, response).into())
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}
