use crate::constants::{REFRESH_INTERVAL, TOKEN_LENGTH, TOKEN_LIFETIME};
use crate::data::models::user::User;
use crate::data::models::user_token::UserToken;
use crate::error::{ServerError, ServerResult};
use crate::services::crypto::{hash_token, random_string, verify_hash};
use chrono::Utc;
use hyper::header::REFRESH;
use sqlx::PgPool;

pub async fn issue_token(conn: &PgPool, user_id: i32) -> ServerResult<String> {
    let secret = random_string(TOKEN_LENGTH);
    let digest = hash_token(secret.clone())?;
    let key = nanoid::nanoid!();

    let now = Utc::now();
    let token_expiry = now + *TOKEN_LIFETIME;

    // Form the string that the user will store and use
    let credential = format!("{}.{}", key, secret);

    // Put the token in the database
    UserToken::create(conn, key, digest, token_expiry, user_id).await?;

    Ok(credential)
}

pub async fn verify_token(conn: &PgPool, raw_token: String) -> ServerResult<User> {
    let (key, secret) = raw_token
        .split_once(".")
        .ok_or(ServerError::MalformedTokenError)?;

    // Search the database for the key, and get the user as well if we find it
    let (db_token, rel_user) = UserToken::get(conn, key).await?;

    // Check the digest on the token
    if !(verify_hash(&db_token.digest, secret))? {
        return Err(ServerError::InvalidTokenError);
    }

    // Check the expiry on the token
    let now = Utc::now();

    if db_token.expiry <= now {
        // If the token is expired, delete it from the database
        UserToken::delete(conn, key).await?;
        return Err(ServerError::ExpiredTokenError);
    }

    // If it's not expired, check whether we should update the expiry
    if db_token.updated_at + *REFRESH_INTERVAL < now {
        UserToken::refresh(conn, key, now + *TOKEN_LIFETIME).await?;
    }

    Ok(rel_user)
}
