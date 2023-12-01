use sqlx::PgPool;
use uuid::Uuid;

use super::ListRow;

#[tracing::instrument(
    name = "Create new daily task list"
    skip(pool)
)]
pub async fn create_new_list(
    pool: &PgPool,
    user_id: Uuid,
    user_name: String,
) -> Result<ListRow, sqlx::Error> {
    // TODO: Set new list name as current date
    let list_name = "New Daily Task List";
    let description = format!("New Daily Task List for {}", user_name);

    let result = sqlx::query_as!(
        ListRow,
        r#"
        INSERT INTO lists (
            list_id,
            name,
            created_by_user_id,
            is_daily_task_list,
            description
        )
        VALUES($1, $2, $3, TRUE, $4)
        RETURNING 
            list_id,
            name,
            description,
            created_date;"#,
        Uuid::new_v4(),
        list_name,
        &user_id,
        description,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result)
}
