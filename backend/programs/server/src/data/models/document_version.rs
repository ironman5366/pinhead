use serde_json::Value;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Json;

#[derive(sqlx::FromRow, Debug)]
pub struct DocumentVersion {
    pub id: i32,
    pub content: Json<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
