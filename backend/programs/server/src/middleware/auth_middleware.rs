use crate::data::models::user::User;
use crate::error::{ServerError, ServerResult};
use crate::services::token::verify_token;
use crate::state::State;
use axum::extract::FromRequest;
use axum::http::HeaderValue;
use axum::{
    http,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use std::sync::Arc;

async fn get_user_from_header(auth_header_raw: String, state: State) -> ServerResult<User> {
    let token = auth_header_raw
        .split_whitespace()
        .last()
        .ok_or(ServerError::UnauthorizedError)?
        .into();

    let user = verify_token(&state.db_pool, token).await?;
    Ok(user)
}

/// Expect to pull out a "Bearer {token}" string from the Authorization header
pub async fn auth_middleware<T>(mut req: Request<T>, next: Next<T>) -> ServerResult<Response> {
    let auth_headers = req.headers().get(http::header::AUTHORIZATION);

    let auth_header = if let Some(header_val) = auth_headers {
        header_val.to_str()?
    } else {
        return Err(ServerError::BadRequestError(
            "Authorization Header Required".into(),
        ));
    };

    let state = match req.extensions().get::<Arc<State>>() {
        Some(state) => state,
        None => {
            return Err(ServerError::InternalServerError(
                "Failed to access shared database connection".into(),
            ))
        }
    };

    Ok(next.run(req).await)
}
