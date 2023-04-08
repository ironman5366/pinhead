use crate::data::models::document_version::DocumentVersion;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{query_as, PgPool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl Document {
    /// Get the latest version of this document's content
    pub async fn content(&self, conn: &PgPool) -> ServerResult<DocumentVersion> {
        Ok(query_as!(
            DocumentVersion,
            r#"SELECT * FROM document_versions WHERE document_id=$1 ORDER BY created_at LIMIT 1"#,
            self.id
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn get(conn: &PgPool, id: i32) -> ServerResult<Self> {
        Ok(
            query_as!(Document, r#"SELECT * FROM documents WHERE id=$1"#, id)
                .fetch_one(conn)
                .await?,
        )
    }

    pub async fn list(conn: &PgPool) -> ServerResult<Vec<Self>> {
        Ok(query_as!(Document, r#"SELECT * FROM documents"#)
            .fetch_all(conn)
            .await?)
    }

    pub async fn create(conn: &PgPool, title: String, content: Value) -> ServerResult<Self> {
        Ok(query_as!(
            Document,
            r#"
                WITH new_document AS (
                    INSERT INTO documents(title)
                    VALUES ($1)
                    RETURNING *
                ),
                document_version_insert AS (
                    INSERT INTO document_versions (document_id, content)
                    SELECT id, $2::jsonb
                    FROM new_document
                )
                SELECT * FROM new_document;
            "#,
            title,
            content
        )
        .fetch_one(conn)
        .await?)
    }
}
