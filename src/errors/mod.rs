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

    #[error("Service Unvailable: {0}")]
    ServiceUnvailable(String),

    // --- Infrastructure errors ---
    #[error("Database error")]
    Database(#[from] sqlx::Error),

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
