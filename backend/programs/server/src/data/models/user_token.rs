use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct UserToken {
    pub key: String,
    pub user_id: i32,
    pub digest: String,
    pub expiry: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

