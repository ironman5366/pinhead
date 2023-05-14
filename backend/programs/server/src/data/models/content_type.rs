use crate::data::models::content_field::ContentField;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct ContentType {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub collection: bool,
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

    pub async fn get_by_code(conn: &PgPool, code: String) -> ServerResult<Self> {
        Ok(query_as!(
            ContentType,
            "SELECT * FROM content_types WHERE code=$1",
            code
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn list(conn: &PgPool) -> ServerResult<Vec<Self>> {
        Ok(query_as!(ContentType, "SELECT * FROM content_types;")
            .fetch_all(conn)
            .await?)
    }

    pub async fn create(
        conn: &PgPool,
        name: String,
        code: String,
        field_ids: Vec<i32>,
    ) -> ServerResult<Self> {
        let mut transaction = conn.begin().await?;

        // Insert a new content type
        let content_type = query_as!(
            ContentType,
            r#"
            INSERT INTO content_types (name, code)
                VALUES ($1, $2)
            RETURNING *;
            "#,
            name,
            code
        )
        .fetch_one(&mut transaction)
        .await?;

        // TODO: provide ranks in the serializer
        let field_ranks: Vec<i32> = (1..=field_ids.len() as i32).collect();

        // Batch insert associated content type fields
        query!(
            r#"
            INSERT INTO content_type_fields (content_type_id, content_field_id, field_rank)
                SELECT $1, unnest($2::integer[]), unnest($3::integer[])
            "#,
            content_type.id,
            &field_ids,
            &field_ranks,
        )
        .execute(&mut transaction)
        .await?;

        // Commit the transaction
        transaction.commit().await?;

        Ok(content_type)
    }
}
