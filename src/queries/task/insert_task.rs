use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::InsertTaskRequest;

#[tracing::instrument(
    name = "Insert Task",
    skip(data, pool)
)]
pub async fn insert_task(
    data: InsertTaskRequest,
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "INSERT INTO tasks (
            task_id,
            name,
            has_child_tasks,
            created_by_user_id,
            parent_list_id,
            notes,
            due_date
        )
        VALUES($1, $2, $3, $4, $5, $6, $7)",
        Uuid::new_v4(),
        data.name,
        false,
        user_id,
        data.parent_list_id,
        data.notes,
        data.due_date,
    )
    .execute(pool)
    .await
    .context("Failed to insert new task")?;

    Ok(())
}