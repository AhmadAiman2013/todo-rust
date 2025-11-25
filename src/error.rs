use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeaderError {
    #[error("Missing header: {0}")]
    MissingHeader(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation err: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Middleware error: {0}")]
    MiddlewareError(#[from] HeaderError),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::MiddlewareError(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        let message = match self {
            AppError::DatabaseError(ref e) => {
                log::error!("Database error: {:?}", e);
                "An unexpected error occurred".to_string()
            }
            _ => self.to_string(),
        };

        let body = Json(ErrorResponse {
            error: message,
        });
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
