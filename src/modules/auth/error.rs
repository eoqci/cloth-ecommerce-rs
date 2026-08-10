use crate::errors::AppError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Login Failed")]
    AuthorizationFailed,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid refresh token")]
    InvalidRefreshToken,
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        // let msg = err.to_string();
        match err {
            AuthError::UserAlreadyExists => AppError::Conflict(err.to_string()),
            AuthError::UserNotFound => AppError::NotFound {
                resource: "User".to_string(),
            },
            AuthError::AuthorizationFailed => AppError::Unauthorized(err.to_string()),
            AuthError::InvalidToken
            | AuthError::TokenExpired
            | AuthError::SessionExpired
            | AuthError::InvalidRefreshToken => AppError::Unauthorized(err.to_string()),
        }
    }
}
