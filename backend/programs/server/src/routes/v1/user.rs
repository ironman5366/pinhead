use crate::data::models::user::User;
use crate::error::{ServerError, ServerResult};
use crate::serializers::user::UserSerializer;
use crate::services::crypto::verify_hash;
use crate::services::token::issue_token;
use crate::state::State;
use axum::{Extension, Json};
use hyper::Response;
use serde::{Deserialize, Serialize};
use serde_json::value::Index;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct RegisterLoginSerializer {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterLoginResponseSerializer {
    pub token: String,
}

#[axum_macros::debug_handler]
pub async fn register(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<RegisterLoginSerializer>,
) -> ServerResult<Json<RegisterLoginResponseSerializer>> {
    let new_user = User::create(&state.db_pool, payload.email, payload.password).await?;
    log::info!("Registered user {}", new_user.email);

    // If the user registered successfully, issue a token for them
    let token = issue_token(&state.db_pool, new_user.id).await?;

    Ok(Json(RegisterLoginResponseSerializer { token }))
}

#[axum_macros::debug_handler]
pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<RegisterLoginSerializer>,
) -> ServerResult<Json<RegisterLoginResponseSerializer>> {
    // TODO: better error for not found, rate limiting

    // Try to get a user with the given email
    let rel_user = User::get_by_email(&state.db_pool, payload.email).await?;

    // Check the user password
    if !verify_hash(&rel_user.password, &payload.password)? {
        return Err(ServerError::InvalidPasswordError);
    }

    log::info!("Logged in user {}", rel_user.email);

    // If the user is logged in successfully, issue a token for them
    let token = issue_token(&state.db_pool, rel_user.id).await?;

    Ok(Json(RegisterLoginResponseSerializer { token }))
}

#[axum_macros::debug_handler]
pub async fn current(
    Extension(user): Extension<User>,
    Extension(_): Extension<Arc<State>>,
) -> ServerResult<Json<UserSerializer>> {
    Ok(Json(user.into()))
}
