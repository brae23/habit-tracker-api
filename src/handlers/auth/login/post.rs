use crate::authentication::{validate_credentials, Credentials, TypedSession};
use crate::domain::{AuthError, LoginError};
use actix_web::error::InternalError;
use actix_web::web;
use actix_web::HttpResponse;

use sqlx::PgPool;

#[tracing::instrument(
    skip(body, pool, session)
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    body: web::Json<Credentials>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let username = body.0.username;
    let credentials = Credentials {
        username: username.clone(),
        password: body.0.password,
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            let _ = session.insert_user_id(user_id).map_err(|e| {
                let error = LoginError::UnexpectedError(e.into());
                InternalError::from_response(error, HttpResponse::InternalServerError().finish())
            });
            let body = UserResponse {
                user_id,
                user_name: username,
            };
            Ok(HttpResponse::Ok().json(body))
        }
        Err(e) => match e {
            AuthError::InvalidCredentials(_) => {
                let error = LoginError::AuthError(e.into());
                Err(InternalError::from_response(
                    error,
                    HttpResponse::BadRequest().finish(),
                ))
            }
            AuthError::UnexpectedError(_) => {
                let error = LoginError::UnexpectedError(e.into());
                Err(InternalError::from_response(
                    error,
                    HttpResponse::InternalServerError().finish(),
                ))
            }
        },
    }
}

#[derive(serde::Serialize)]
struct UserResponse {
    pub user_id: uuid::Uuid,
    pub user_name: String,
}
