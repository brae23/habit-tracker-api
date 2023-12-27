use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub name: String,
    pub completed: bool,
    pub created_by_user_id: uuid::Uuid,
    pub has_child_tasks: bool,
    pub created_date: DateTime<Utc>,
    pub parent_list_id: uuid::Uuid,
    pub completed_by_user_id: Option<uuid::Uuid>,
    pub notes: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub due_date: Option<DateTime<Utc>>,
}
