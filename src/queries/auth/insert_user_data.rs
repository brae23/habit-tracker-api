use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{authentication::compute_password_hash, telemetry::spawn_blocking_with_tracing};

#[tracing::instrument(name = "Insert new user data", skip(email, password, pool))]
pub async fn insert_user_data(
    user_name: &str,
    email: &str,
    password: Secret<String>,
    pool: &PgPool,
) -> Result<uuid::Uuid, anyhow::Error> {
    let user_id = uuid::Uuid::new_v4();
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    sqlx::query!(
        "INSERT INTO users (user_id, user_name, email, password_hash)
        VALUES($1, $2, $3, $4)",
        user_id,
        user_name,
        email,
        password_hash.expose_secret(),
    )
    .execute(pool)
    .await
    .context("Failed to insert user data into users table")?;

    Ok(user_id)
}
