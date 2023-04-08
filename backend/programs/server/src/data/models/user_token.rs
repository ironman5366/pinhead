use crate::data::models::user::User;
use crate::error::ServerResult;
use chrono::{DateTime, Utc};
use sqlx::{query, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct UserToken {
    pub key: String,
    pub user_id: i32,
    pub digest: String,
    pub expiry: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserToken {
    pub async fn get(conn: &PgPool, key: &str) -> ServerResult<(UserToken, User)> {
        let res = query!(
            "SELECT user_tokens.key as tokens_key,
                    user_tokens.user_id as tokens_user_id,
                    user_tokens.digest as tokens_digest,
                    user_tokens.expiry as tokens_expiry,
                    user_tokens.created_at as tokens_created_at,
                    user_tokens.updated_at as tokens_updated_at,
                    users.id as users_id,
                    users.email as users_email,
                    users.password as users_password,
                    users.created_at as users_created_at,
                    users.updated_at as users_updated_at
               FROM user_tokens JOIN users
                ON users.id = user_tokens.user_id
            WHERE user_tokens.key=$1
            ",
            key
        )
        .fetch_one(conn)
        .await?;

        let user = User {
            id: res.users_id,
            email: res.users_email,
            password: res.users_password,
            created_at: res.users_created_at,
            updated_at: res.users_updated_at,
        };

        let token = UserToken {
            key: res.tokens_key,
            user_id: res.tokens_user_id,
            digest: res.tokens_digest,
            expiry: res.tokens_expiry,
            created_at: res.tokens_created_at,
            updated_at: res.tokens_updated_at,
        };

        Ok((token, user))
    }

    pub async fn create(
        conn: &PgPool,
        key: String,
        digest: String,
        expiry: DateTime<Utc>,
        user_id: i32,
    ) -> ServerResult<()> {
        query!(
            "INSERT INTO user_tokens (key, digest, expiry, user_id) VALUES ($1, $2, $3, $4)",
            key,
            digest,
            expiry,
            user_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn delete(conn: &PgPool, key: &str) -> ServerResult<()> {
        query!("DELETE FROM user_tokens WHERE key=$1", key)
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn refresh(conn: &PgPool, key: &str, new_expiry: DateTime<Utc>) -> ServerResult<()> {
        let now = Utc::now();
        query!(
            "UPDATE user_tokens SET expiry=$1, updated_at=$2 WHERE key=$3",
            new_expiry,
            now,
            key
        )
        .execute(conn)
        .await?;
        Ok(())
    }
}
