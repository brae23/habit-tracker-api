use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials, UserId},
    utils::e500,
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
        // Update to send with bad request response

        // FlashMessage::error(
        //     "You entered two different new passwords - the field values must match.",
        // )
        // .send();
        return Ok(HttpResponse::BadRequest().finish());
    }

    if form.new_password.expose_secret().len() < 12 {
        // Update to send with bad request response

        // FlashMessage::error(
        //     "The new password is too short - it must be between 12 and 128 characters long.",
        // )
        // .send();
        return Ok(HttpResponse::BadRequest().finish());
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                // Update to send with bad request response

                // FlashMessage::error("The current password is incorrect.").send();
                Ok(HttpResponse::BadRequest().finish())
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    // Update to send with bad request response
    // FlashMessage::error("Your password has been changed.").send();

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_name
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.user_name)
}
