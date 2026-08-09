use super::{AppError, FieldError, ValidationError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

/// Cấu trúc JSON trả về thống nhất
#[derive(serde::Serialize)]
struct ErrorBody {
    error: ErrorDetail,
}

#[derive(serde::Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<FieldError>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, fields): (
            StatusCode,
            &'static str,
            String,
            Option<Vec<FieldError>>,
        ) = match self {
            // 4xx — client error
            AppError::NotFound { resource } => (StatusCode::NOT_FOUND, "NOT_FOUND", resource, None),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg, None),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg, None),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg, None),
            AppError::Validation(v) => match v {
                ValidationError::Field { field, message } => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "VALIDATION_ERROR",
                    "Validation failed".to_string(),
                    Some(vec![FieldError { field, message }]),
                ),
                ValidationError::Multiple(errs) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "VALIDATION_ERROR",
                    "Validation failed".to_string(),
                    Some(errs),
                ),
            },
            // 5xx — lỗi server (log nhưng không leak detail)
            AppError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg,
                None,
            ),
            AppError::Config(e) => {
                error!(error = %e, "Configuration error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "CONFIG_ERROR",
                    "A server configuration error occurred".to_string(),
                    None,
                )
            }
            AppError::Database(e) => {
                error!(error = %e, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred".to_string(),
                    None,
                )
            }
            AppError::ExternalService { service, source } => {
                error!(%service, error = %source, "External service error");
                (
                    StatusCode::BAD_GATEWAY,
                    "EXTERNAL_SERVICE_ERROR",
                    format!("Service '{service}' unavailable"),
                    None,
                )
            }
            AppError::Internal(e) => {
                error!(error = %e, "Internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An unexpected error occurred".to_string(),
                    None,
                )
            }
        };

        let body = Json(ErrorBody {
            error: ErrorDetail {
                code,
                message,
                fields,
            },
        });

        (status, body).into_response()
    }
}
