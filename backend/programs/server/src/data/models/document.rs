use crate::error::ServerResult;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{query_as, PgPool};

#[derive(sqlx::FromRow, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
    pub async fn get(conn: &PgPool, id: i32) -> ServerResult<Document> {
        Ok(
            query_as!(Document, r#"SELECT * FROM documents WHERE id=$1"#, id)
                .fetch_one(conn)
                .await?,
        )
    }

    pub async fn create(conn: &PgPool, title: String) -> ServerResult<Document> {
        Ok(query_as!(
            Document,
            r#"INSERT into documents (title) VALUES ($1) RETURNING *"#,
            title
        )
        .fetch_one(conn)
        .await?)
    }
}
