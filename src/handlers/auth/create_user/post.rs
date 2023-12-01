use actix_web::{error::InternalError, web, HttpResponse};
use sqlx::PgPool;
use validator::validate_email;

use crate::{
    authentication::{validate_password_length, TypedSession},
    domain::{to_bad_request_response, CreateUserData, CreateUserError, UserResponse},
    queries::{insert_user_data, user_exists},
};

#[tracing::instrument(name = "Create user account", skip(data, pool, session))]
pub async fn create_user(
    data: web::Json<CreateUserData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<CreateUserError>> {
    validate_password_length(&data.password).map_err(|(e, r)| {
        let error = CreateUserError::InvalidPassword(e);
        InternalError::from_response(error, r)
    })?;

    validate_username_has_value(&data.0.user_name)?;

    if !validate_email(&data.0.email) {
        let error = CreateUserError::InvalidEmail;
        let reason = "Email is invalid";
        return Err(InternalError::from_response(
            error,
            to_bad_request_response(reason),
        ));
    }

    validate_user_does_not_exist(&data.0.user_name, &pool).await?;

    let user_id = insert_user_data(&data.0.user_name, &data.0.email, data.0.password, &pool)
        .await
        .map_err(|e| {
            let error = CreateUserError::UserInsertError(e);
            InternalError::from_response(error, HttpResponse::InternalServerError().finish())
        })?;

    session.renew();
    session.insert_user_id(user_id).map_err(|e| {
        let error = CreateUserError::StartSessionError(e.into());
        session.log_out();
        InternalError::from_response(error, HttpResponse::InternalServerError().finish())
    })?;

    let body = UserResponse {
        user_id,
        user_name: data.0.user_name,
    };

    Ok(HttpResponse::Ok().json(body))
}

async fn validate_user_does_not_exist(
    user_name: &str,
    pool: &PgPool,
) -> Result<(), InternalError<CreateUserError>> {
    let user_exists = user_exists(user_name, pool).await.map_err(|e| {
        let error = CreateUserError::UnexpectedError(e);
        InternalError::from_response(error, HttpResponse::InternalServerError().finish())
    })?;

    if user_exists {
        let error = CreateUserError::UserAlreadyExists;
        let reason = "User already exists";
        return Err(InternalError::from_response(
            error,
            to_bad_request_response(reason),
        ));
    }

    Ok(())
}

fn validate_username_has_value(user_name: &str) -> Result<(), InternalError<CreateUserError>> {
    if user_name.is_empty() {
        let error = CreateUserError::InvalidUsername;
        let reason = "Username must have a value";
        return Err(InternalError::from_response(
            error,
            to_bad_request_response(reason),
        ));
    }
    Ok(())
}
