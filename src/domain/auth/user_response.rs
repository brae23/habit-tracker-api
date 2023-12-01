#[derive(serde::Serialize)]
pub struct UserResponse {
    pub user_id: uuid::Uuid,
    pub user_name: String,
}
