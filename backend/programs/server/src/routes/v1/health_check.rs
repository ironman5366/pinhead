use crate::state::State;
use axum::Extension;
use std::sync::Arc;

pub async fn health_check(Extension(state): Extension<Arc<State>>) -> &'static str {
    "OK"
}