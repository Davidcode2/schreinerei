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

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Auth(_) | AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
        }
    }

    fn log_database_error(&self) {
        if let AppError::Database(msg) = self {
            tracing::error!("Database error: {}", msg);
        }
    }

    fn log_internal_error(&self) {
        if let AppError::Internal(msg) = self {
            tracing::error!("Internal error: {}", msg);
        }
    }

    fn response_message(self) -> String {
        match self {
            AppError::Database(_) => "Database error".to_string(),
            AppError::Internal(_) => "Internal server error".to_string(),
            AppError::Auth(msg)
            | AppError::NotFound(msg)
            | AppError::Validation(msg)
            | AppError::Forbidden(msg)
            | AppError::Unauthorized(msg)
            | AppError::Conflict(msg) => msg,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.log_database_error();
        self.log_internal_error();
        let status = self.status_code();
        let message = self.response_message();

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
