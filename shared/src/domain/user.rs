use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub hash: String,
    pub created_at: DateTime<Utc>,
}