use axum::Json;
use crate::response::ApiResponse;


pub async fn health() -> Json<ApiResponse<String>> {
   Json(ApiResponse::success_with_message(
         "todo-rust".to_string(),
         "Healthy"
   ))
}