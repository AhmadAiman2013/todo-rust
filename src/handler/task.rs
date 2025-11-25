use axum::extract::{State};
use axum::{Extension, Json};
use validator::Validate;
use crate::error::AppResult;
use crate::extractor::{SafeJson, SafePath};
use crate::model::{CreateTaskRequest, Task};
use crate::response::ApiResponse;
use crate::state::AppState;

pub async fn list_task_by_id(
    State(state) : State<AppState>,
    SafePath(id): SafePath<u64>
) -> AppResult<Json<ApiResponse<Task>>> {
    let task = state.task.get_task_by_id(id).await?;
    Ok(Json(ApiResponse::success(task)))
}

pub async fn list_tasks_by_user_id(
    State(state) : State<AppState>,
    Extension(user_id): Extension<String>,
) -> AppResult<Json<ApiResponse<Vec<Task>>>> {
    let tasks = state.task.get_tasks_by_user_id(&user_id).await?;
    Ok(Json(ApiResponse::success(tasks)))
}

pub async fn create_task(
    State(state) : State<AppState>,
    Extension(user_id): Extension<String>,
    SafeJson(request): SafeJson<CreateTaskRequest>,
) -> AppResult<Json<ApiResponse<u64>>> {
    request.validate()?;
    let task_id = state.task.add_task(&user_id, &request).await?;
    Ok(Json(ApiResponse::success(task_id)))
}

