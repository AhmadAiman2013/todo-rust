use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ENUM('NONE', 'DAILY', 'WEEKLY', 'MONTHLY')")]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskRecurrence {
   #[sqlx(rename = "NONE")]
    None,

    #[sqlx(rename = "DAILY")]
    Daily,

    #[sqlx(rename = "WEEKLY")]
    Weekly,

    #[sqlx(rename = "MONTHLY")]
    Monthly
}


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: u64,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub status: bool,
    pub recurrence_rule : TaskRecurrence,
    pub recurrence_start_date: Option<NaiveDateTime>,
    pub recurrence_end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1))]
    pub title: String,

    pub description: Option<String>,

    pub due_date: Option<NaiveDateTime>,

    pub recurrence_rule : TaskRecurrence,

    pub recurrence_start_date: Option<NaiveDateTime>,

    pub recurrence_end_date: Option<NaiveDateTime>,
}

// #[derive(Debug, Clone, Serialize)]
// pub struct Task {
//     pub id: u64,
//     pub user_id: String,
//     pub title: String,
//     pub description: Option<String>,
//     pub due_date: Option<NaiveDateTime>,
//     pub status: bool,
//     pub recurrence_rule : TaskRecurrence,
//     pub recurrence_start_date: Option<NaiveDateTime>,
//     pub recurrence_end_date: Option<NaiveDateTime>,
// }

// impl From<TaskEntity> for Task {
//     fn from(entity: TaskEntity) -> Self {
//         Self {
//             id: entity.id,
//             user_id: entity.user_id,
//             title: entity.title,
//             description: entity.description,
//             due_date: entity.due_date,
//             status: entity.status,
//             recurrence_rule: entity.recurrence_rule,
//             recurrence_start_date: entity.recurrence_start_date,
//             recurrence_end_date: entity.recurrence_end_date,
//         }
//     }
// }