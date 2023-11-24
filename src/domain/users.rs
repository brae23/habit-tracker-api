use std::ops::Deref;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

pub struct User {
    pub user_id: UserId,
    pub user_name: String,
    pub account_created_on: DateTime<Utc>,
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}