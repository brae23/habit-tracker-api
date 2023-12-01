use sqlx::PgPool;

use crate::domain::Task;

#[tracing::instrument(
    name = "Get task data for daily task list"
    skip(pool, parent_list_id)
)]
pub async fn get_task_data(
    pool: &PgPool,
    parent_list_id: uuid::Uuid,
) -> Result<Option<Vec<Task>>, sqlx::Error> {
    let result = sqlx::query_as!(
        Task,
        r#"
        SELECT *
        FROM tasks
        WHERE parent_list_id = $1;
        "#,
        parent_list_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(Some(result))
}
