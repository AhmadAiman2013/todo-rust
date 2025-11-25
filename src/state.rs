use sqlx::MySqlPool;
use crate::repository::TaskRepository;
use crate::service::TaskService;

#[derive(Clone)]
pub struct AppState {
    pub task: TaskService
}

impl AppState {
    pub fn new(pool: MySqlPool) -> Self {
        let task_repo = TaskRepository::new(pool);
        let task = TaskService::new(task_repo);
        Self { task }
    }
}