use crate::error::ServerError::DatabaseError;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::Error::Database;
use sqlx::{query, query_as, PgPool};

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

    pub async fn create(
        conn: &PgPool,
        name: Option<String>,
        code: String,
        schema: Value,
    ) -> ServerResult<Self> {
        Ok(query_as!(
            ContentField,
            "INSERT INTO content_fields(name, code, system, schema) VALUES ($1, $2, $3, $4) RETURNING *",
            name,
            code,
            // Always user created
            false,
            schema
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn code_available(conn: &PgPool, code: String) -> ServerResult<bool> {
        let exists = query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM content_fields
                    WHERE code = $1
                );
               "#,
            code
        )
        .fetch_one(conn)
        .await?
        .exists
        .ok_or(DatabaseError);

        Ok(!exists?)
    }
}
