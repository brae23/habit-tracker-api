use chrono::{DateTime, Utc};

pub struct User {
    pub user_id: uuid::Uuid,
    pub user_name: String,
    pub account_created_on: DateTime<Utc>,
}
