use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct PasswordChangeRequest {
    pub current_password: Secret<String>,
    pub new_password: Secret<String>,
    pub new_password_check: Secret<String>,
}
