use crate::data::models::document::Document;
use crate::state::State;
use axum::extract::Path;
use axum::Extension;

pub async fn get_document(Extension(state): Extension<State>, Path(document_id): Path<i32>) {}
