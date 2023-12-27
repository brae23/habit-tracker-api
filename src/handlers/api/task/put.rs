use actix_web::{HttpResponse, web, error::InternalError};
use sqlx::PgPool;

use crate::{domain::UpdateTaskRequest, authentication::UserId, queries::update_task_data};

#[tracing::instrument(
    name = "Update Task",
    skip(data, pool)
)]
pub async fn update_task(
    data: web::Json<UpdateTaskRequest>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    // Update task
    update_task_data(data.0, &pool, user_id)
        .await
        .map_err(|e| InternalError::from_response(e, HttpResponse::InternalServerError().finish()))?;

    Ok(HttpResponse::Ok().finish())
}