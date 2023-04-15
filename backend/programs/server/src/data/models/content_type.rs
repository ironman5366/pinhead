use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ContentType {
    pub id: i32,
    pub name: String,
    // Foreign keys to content field objects
    pub field_ids: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
