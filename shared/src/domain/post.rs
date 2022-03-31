use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub preview_content: String,
    pub full_content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub world_id: i32,
}