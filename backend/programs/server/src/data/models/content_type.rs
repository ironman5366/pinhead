use crate::data::models::content_field::ContentField;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct ContentType {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ContentType {
    pub async fn fields(&self, conn: &PgPool) -> ServerResult<Vec<ContentField>> {
        Ok(query_as!(
            ContentField,
            r#"
            SELECT cf.*
                FROM content_type_fields ctf
                JOIN content_fields cf ON ctf.content_field_id = cf.id
                WHERE ctf.content_type_id = $1
            ORDER BY ctf.field_rank;
            "#,
            self.id
        )
        .fetch_all(conn)
        .await?)
    }

    pub async fn list(conn: &PgPool) -> ServerResult<Vec<ContentType>> {
        Ok(query_as!(ContentType, "SELECT * FROM content_types;")
            .fetch_all(conn)
            .await?)
    }
}
