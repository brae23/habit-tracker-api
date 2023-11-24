use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    authentication::UserId,
    domain::{List, Task},
    utils::e500,
};

#[tracing::instrument(
    name = "Get daily task list for user using user_id and date"
    skip(pool)
)]
pub async fn get_daily_task_list(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let list_row_data = get_list_data(&pool, **user_id).await.map_err(e500)?;

    let task_data = get_task_data(&pool, list_row_data.list_id)
        .await
        .map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(List {
            list_id: list_row_data.list_id,
            name: list_row_data.name,
            description: list_row_data.description,
            created_date: list_row_data.created_date,
            list_items: task_data,
        }))
}

#[tracing::instrument(
    name = "Get list_id using user_id and date"
    skip(pool, user_id)
)]
async fn get_list_data(pool: &PgPool, user_id: Uuid) -> Result<ListRow, sqlx::Error> {
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
            let result = create_new_list(pool, user_id).await?;
            Ok(result)
        }
    }
}

#[tracing::instrument(
    name = "Create list using user_id"
    skip(pool, user_id)
)]
async fn create_new_list(pool: &PgPool, user_id: Uuid) -> Result<ListRow, sqlx::Error> {
    // Set new list name
    let list_name = "list name";

    let result = sqlx::query_as!(
        ListRow,
        r#"
        INSERT INTO lists (
            list_id,
            name,
            created_by_user_id,
            is_daily_task_list
        )
        VALUES($1, $2, $3, TRUE)
        RETURNING 
            list_id,
            name,
            description,
            created_date;"#,
        Uuid::new_v4(),
        list_name,
        &user_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result)
}

#[tracing::instrument(
    name = "Get task data from list_id"
    skip(pool, parent_list_id)
)]
async fn get_task_data(
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

#[derive(sqlx::FromRow)]
struct ListRow {
    list_id: uuid::Uuid,
    name: String,
    description: Option<String>,
    created_date: DateTime<Utc>,
}
