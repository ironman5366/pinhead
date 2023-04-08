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

#[axum_macros::debug_handler]
pub async fn register(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<RegisterLoginSerializer>,
) -> ServerResult<Json<UserSerializer>> {
    let new_user = User::create(&state.db_pool, payload.email, payload.password).await?;
    Ok(Json(new_user.into()))
}

#[derive(Serialize, Debug)]
pub struct LoginResponseSerializer {
    pub user: UserSerializer,
    pub token: String,
}

#[axum_macros::debug_handler]
pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<RegisterLoginSerializer>,
) -> ServerResult<Json<LoginResponseSerializer>> {
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

    return Ok(Json(LoginResponseSerializer {
        user: rel_user.into(),
        token,
    }));
}

#[axum_macros::debug_handler]
pub async fn current(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<UserSerializer>> {
    // TODO: figure out what's up with the auth middleware, this should take an Extension<User>
    Ok(Json(
        User::get_by_email(&state.db_pool, "will@example.com".to_string())
            .await?
            .into(),
    ))
}
