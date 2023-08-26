use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("User not found")]
    UserNotFound,
    #[error("Connection timed out")]
    ConnectionTimeout,
}
