use crate::data::models::document::Document;
use crate::data::types::content::Content;
use crate::error::{ServerError, ServerResult};
use serde_json::Value;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{query_as, PgPool};
use std::collections::HashMap;

#[derive(sqlx::FromRow, Debug)]
pub struct DocumentVersion {
    pub id: i32,
    pub content: Value,
    pub document_id: i32,
    pub data: HashMap<String, Option<DataField>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DocumentVersion {
    pub async fn get(conn: &PgPool, id: i32) -> ServerResult<DocumentVersion> {
        Ok(query_as!(
            DocumentVersion,
            r#"SELECT * FROM document_versions WHERE id=$1"#,
            id
        )
        .fetch_one(conn)
        .await?)
    }
}
