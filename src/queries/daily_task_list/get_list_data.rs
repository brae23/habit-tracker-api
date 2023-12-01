use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::create_new_list;

#[tracing::instrument(
    name = "Get daily task list data"
    skip(pool)
)]
pub async fn get_list_data(
    pool: &PgPool,
    user_id: Uuid,
    user_name: String,
) -> Result<ListRow, sqlx::Error> {
    let result: Option<ListRow> = sqlx::query_as!(
        ListRow,
        r#"
        SELECT
        list_id,
        name,
        description,
        created_date
        FROM lists
        WHERE (created_by_user_id = $1 AND
            is_daily_task_list = TRUE)
        "#,
        user_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    match result {
        Some(res) => Ok(res),
        None => {
            let result = create_new_list(pool, user_id, user_name).await?;
            Ok(result)
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct ListRow {
    pub list_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_date: DateTime<Utc>,
}
