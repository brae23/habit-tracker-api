use actix_web::{http::header::ContentType, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    authentication::UserId,
    domain::{e500, List},
    queries::{get_list_data, get_task_data, get_username},
};

#[tracing::instrument(
    name = "Get daily task list for user"
    skip(pool)
)]
pub async fn get_daily_task_list(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_name = get_username(**user_id, &pool).await.map_err(e500)?;

    let list_row_data = get_list_data(&pool, **user_id, user_name)
        .await
        .map_err(e500)?;

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
