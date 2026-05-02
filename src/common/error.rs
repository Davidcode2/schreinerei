use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Validation(msg)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
