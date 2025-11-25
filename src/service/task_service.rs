use crate::error::AppResult;
use crate::model::{CreateTaskRequest, Task};
use crate::repository::TaskRepository;

#[derive(Clone)]
pub struct TaskService {
    repo: TaskRepository
}

impl TaskService {
    pub fn new(repo: TaskRepository) -> Self {
        Self { repo }
    }

    pub async fn get_task_by_id(&self, task_id: u64) -> AppResult<Task> {
        let entity = self.repo.find_task_by_id(task_id).await?;
        Ok(entity)
    }

    pub async fn get_tasks_by_user_id(&self, user_id: &str) -> AppResult<Vec<Task>> {
        let entities = self.repo.find_all_tasks_based_on_user_id(user_id).await?;
        Ok(entities)
    }
    
    pub async fn add_task(&self, user_id: &str, request: &CreateTaskRequest) -> AppResult<u64> {
        let task_id = self.repo.create_task(user_id, request).await?;
        Ok(task_id)
    }


}