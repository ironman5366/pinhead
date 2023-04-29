use crate::data::models::document_version::DocumentVersion;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{query_as, PgPool};

#[derive(sqlx::FromRow, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub content_type_id: i32,
    pub created_at: DateTime<Utc>,
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

    pub async fn create(
        conn: &PgPool,
        title: String,
        content_type_id: i32,
        content: Value,
    ) -> ServerResult<Self> {
        Ok(query_as!(
            Document,
            r#"
                WITH new_document AS (
                    INSERT INTO documents(title, content_type_id)
                    VALUES ($1, $2)
                    RETURNING *
                ),
                document_version_insert AS (
                    INSERT INTO document_versions (document_id, content)
                    SELECT id, $3::jsonb
                    FROM new_document
                )
                SELECT * FROM new_document;
            "#,
            title,
            content_type_id,
            content
        )
        .fetch_one(conn)
        .await?)
    }
}
