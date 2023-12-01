use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct CreateUserData {
    pub email: String,
    pub user_name: String,
    pub password: Secret<String>,
}
