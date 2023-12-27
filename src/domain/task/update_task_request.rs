use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UpdateTaskRequest {  
    pub task_id: Uuid,
    pub name: String,
    pub completed: bool,
    pub parent_list_id: Uuid,
    pub notes: Option<String>,
    pub due_date: DateTime<Utc>,
    pub completed_by_user_id: Option<Uuid>,
}