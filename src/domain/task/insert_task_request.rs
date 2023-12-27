use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct InsertTaskRequest {
    pub name: String,
    pub parent_list_id: Uuid,
    pub notes: String,
    pub due_date: DateTime<Utc>
}