use super::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum CreateUserError {
    #[error("Password is invalid")]
    InvalidPassword(#[source] anyhow::Error),
    #[error("Failed to insert new user")]
    UserInsertError(#[source] anyhow::Error),
    #[error("Failed to start session for new user")]
    StartSessionError(#[source] anyhow::Error),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Username must have a value")]
    InvalidUsername,
    #[error("User email is invalid")]
    InvalidEmail,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
