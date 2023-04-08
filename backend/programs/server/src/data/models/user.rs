use crate::error::ServerResult;
use crate::services::crypto::hash_password;
use chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn get(conn: &PgPool, id: i32) -> ServerResult<Self> {
        Ok(query_as!(User, "SELECT * FROM users where id=$1", id)
            .fetch_one(conn)
            .await?)
    }

    pub async fn get_by_email(conn: &PgPool, email: String) -> ServerResult<Self> {
        Ok(query_as!(User, "SELECT * FROM users WHERE email=$1", email)
            .fetch_one(conn)
            .await?)
    }

    pub async fn create(conn: &PgPool, email: String, password: String) -> ServerResult<Self> {
        let hashed_password = hash_password(password)?;
        Ok(query_as!(
            User,
            r#"
                INSERT INTO users(email, password)
                VALUES ($1, $2)
                RETURNING *
                "#,
            email,
            hashed_password
        )
        .fetch_one(conn)
        .await?)
    }
}
