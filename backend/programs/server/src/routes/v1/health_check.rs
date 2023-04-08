use crate::error::ServerResult;
use crate::state::State;
use axum::Extension;
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn health_check(_: Extension<Arc<State>>) -> ServerResult<&'static str> {
    Ok("OK")
}
