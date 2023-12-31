use super::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    #[allow(dead_code)]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
