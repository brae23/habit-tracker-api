use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::session_state::TypedSession;
use crate::utils::error_chain_fmt;
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
    let credentials = Credentials {
        username: body.0.username,
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
            Ok(HttpResponse::Ok().finish())
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

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    #[allow(dead_code)]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
