use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use hyper::Server;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
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

    pub async fn create(conn: &PgPool, title: String) -> ServerResult<Self> {
        Ok(query_as!(
            Document,
            r#"INSERT into documents (title) VALUES ($1) RETURNING *"#,
            title
        )
        .fetch_one(conn)
        .await?)
    }
}
