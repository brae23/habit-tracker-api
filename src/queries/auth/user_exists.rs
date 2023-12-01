use anyhow::Context;
use sqlx::PgPool;

#[tracing::instrument(name = "Check if user exists", skip(pool))]
pub async fn user_exists(user_name: &str, pool: &PgPool) -> Result<bool, anyhow::Error> {
    let exists = sqlx::query!(
        r#"
        SELECT *
        FROM users
        WHERE user_name = $1
        "#,
        user_name,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to get a user")?;

    match exists {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
