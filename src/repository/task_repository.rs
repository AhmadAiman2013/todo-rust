use sqlx::MySqlPool;
use crate::error::{AppError, AppResult};
use crate::model::{CreateTaskRequest, Task, TaskRecurrence};

#[derive(Clone)]
pub struct TaskRepository {
    pool: MySqlPool
}

impl TaskRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool  }
    }

    pub async fn find_task_by_id(&self, task_id: u64) -> AppResult<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id,
                user_id,
                title,
                description,
                due_date,
                (status != 0) as `status: bool`,
                recurrence_rule as `recurrence_rule: TaskRecurrence`,
                recurrence_start_date,
                recurrence_end_date
            FROM tasks
            WHERE id = ?
"#,
            task_id
        )
            .fetch_one(&self.pool)
            .await;

        match task {
            Ok(task) => Ok(task),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound(format!("Task with id {} not found", task_id))),
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }

    pub async fn find_all_tasks_based_on_user_id(&self, user_id: &str) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id,
                user_id,
                title,
                description,
                due_date,
                (status != 0) as `status: bool`,
                recurrence_rule as `recurrence_rule: TaskRecurrence`,
                recurrence_start_date,
                recurrence_end_date
            FROM tasks
            WHERE user_id = ?
"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        if tasks.is_empty() {
            return Err(AppError::NotFound("No tasks found for the given user ID".to_string()));
        }

        Ok(tasks)
    }

    pub async fn create_task(&self, user_id: &str, request: &CreateTaskRequest) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO tasks (user_id, title, description, due_date, recurrence_rule, recurrence_start_date, recurrence_end_date)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            request.title,
            request.description,
            request.due_date,
            request.recurrence_rule as TaskRecurrence,
            request.recurrence_start_date,
            request.recurrence_end_date
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id())
    }


    pub async fn update_task(&self, user_id: &str, task_id: u64, request: &CreateTaskRequest) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE tasks
            SET title = ?, description = ?, due_date = ?, recurrence_rule = ?, recurrence_start_date = ?, recurrence_end_date = ?
            WHERE id = ? AND user_id = ?
            "#,
            request.title,
            request.description,
            request.due_date,
            request.recurrence_rule as TaskRecurrence,
            request.recurrence_start_date,
            request.recurrence_end_date,
            task_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found for user {}", task_id, user_id)));
        }

        Ok(())
         
    }
    
    pub async fn delete_task(&self, task_id: u64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = ?
            "#,
            task_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        Ok(())
    }

}