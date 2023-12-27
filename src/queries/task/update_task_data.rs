use anyhow::Context;
use sqlx::PgPool;

use crate::{domain::UpdateTaskRequest, authentication::UserId};

#[tracing::instrument(
    name = "Update Task Query",
    skip(data, pool)
)]
pub async fn update_task_data(
    data: UpdateTaskRequest,
    pool: &PgPool,
    user_id: UserId
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "
        UPDATE tasks
        SET 
            name = $1,
            completed = $2,
            parent_list_id = $3,
            notes = $4,
            due_date = $5,
            completed_by_user_id = $6
        
        WHERE task_id = $7;
        ",
        data.name,
        data.completed,
        data.parent_list_id,
        data.notes,
        data.due_date,
        data.completed_by_user_id,
        data.task_id,
    )
    .execute(pool)
    .await
    .context("Failed to update task")?;

    Ok(())
}