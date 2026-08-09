pub mod config;
pub mod http;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // --- Domain errors ---
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    // --- Infrastructure errors ---
    // Bỏ #[from]: không cho tự động wrap mọi sqlx::Error thành 500 nữa.
    // Vẫn giữ #[source] để error chain (source()) hoạt động bình thường cho logging/tracing.
    #[error("Database error")]
    Database(#[source] sqlx::Error),
    #[error("External service error: {service}")]
    ExternalService {
        service: String,
        #[source]
        source: anyhow::Error,
    },

    // --- Catch-all ---
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

// `?` ở mọi chỗ gọi sqlx (find_user_by_id, create_session, ...) không cần sửa gì cả -
// vẫn tự convert qua From này như cũ, chỉ khác là giờ map đúng status code thay vì luôn 500.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::RowNotFound = err {
            return AppError::NotFound {
                resource: "Resource".to_string(),
            };
        }

        if let sqlx::Error::Database(db_err) = &err {
            match db_err.code().as_deref() {
                Some("23505") => return AppError::Conflict("Resource already exists".to_string()),
                Some("23503") => {
                    return AppError::Conflict("Referenced resource does not exist".to_string());
                }
                _ => {}
            }
        }

        AppError::Database(err)
    }
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field '{field}': {message}")]
    Field { field: String, message: String },
    #[error("Multiple validation errors")]
    Multiple(Vec<FieldError>),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
