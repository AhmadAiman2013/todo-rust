use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use crate::error::{AppError, HeaderError};
use crate::error::AppError::MiddlewareError;

pub async fn extract_user_info(mut req: Request, next: Next) -> Result<Response, AppError> {
    let user_id_header = req.headers()
        .get("x-user-id")
        .and_then(|value| value.to_str().ok());

    let user_id = if let Some(user_id) = user_id_header {
        user_id.to_string()
    } else {
        return Err(MiddlewareError(HeaderError::MissingHeader("x-user-id".to_string())));
    };

    if user_id.is_empty() {
        return Err(MiddlewareError(HeaderError::InvalidHeader("x-user-id".to_string())));
    }

    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}