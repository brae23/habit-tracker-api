use crate::session_state::TypedSession;
use actix_web::HttpResponse;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    session.log_out();
    Ok(HttpResponse::Ok().finish())
}
