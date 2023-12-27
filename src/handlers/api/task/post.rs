use actix_web::{HttpResponse, web, error::InternalError};
use sqlx::PgPool;

use crate::{authentication::UserId, queries::insert_task, domain::InsertTaskRequest};

#[tracing::instrument(
    name = "Create Task",
    skip(data, pool)
)]
pub async fn create_task(
    data: web::Json<InsertTaskRequest>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    insert_task(data.0, &pool, *user_id)
        .await
        .map_err(|e| InternalError::from_response(e, HttpResponse::InternalServerError().finish()))?;

    Ok(HttpResponse::Ok().finish())
}