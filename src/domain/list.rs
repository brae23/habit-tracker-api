use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::Task;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct List {
    pub list_id: Uuid,
    pub name: String,
    pub created_date: DateTime<Utc>,
    pub description: Option<String>,
    pub list_items: Option<Vec<Task>>,
}