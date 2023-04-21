use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use jsonschema::JSONSchema;
use serde_json::Value;
use sqlx::{query_as, PgPool};

#[derive(sqlx::FromRow, Debug)]
pub struct ContentField {
    pub id: i32,
    pub name: Option<String>,
    // Code is unique across fields
    pub code: String,
    // Whether this is a system type or user created
    pub system: bool,
    // TODO: this should be a JSONSchema type
    pub schema: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ContentField {
    pub async fn list(conn: &PgPool) -> ServerResult<Vec<Self>> {
        Ok(query_as!(ContentField, "SELECT * FROM content_fields;")
            .fetch_all(conn)
            .await?)
    }
}
