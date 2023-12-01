use actix_web::HttpResponse;

use crate::authentication::TypedSession;

#[tracing::instrument(
    name="Logout"
    skip(session)
)]
pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    session.log_out();
    Ok(HttpResponse::Ok().finish())
}
