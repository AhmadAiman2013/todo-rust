use axum::Router;
use axum::routing::{get};
use axum::middleware;
use crate::handler::{create_task, delete_task, health, list_task_by_id, list_tasks_by_user_id, update_task};
use crate::middleware::extract_user_info;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks/{id}", get(list_task_by_id).put(update_task).delete(delete_task))
        .route("/tasks", get(list_tasks_by_user_id).post(create_task))
        .with_state(state)
        .layer(middleware::from_fn(extract_user_info))
}