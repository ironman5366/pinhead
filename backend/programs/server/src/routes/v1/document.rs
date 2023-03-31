use crate::data::models::document::Document;
use crate::error::ServerResult;
use crate::serializers::results::ResultList;
use crate::state::State;
use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::chrono::{DateTime, Utc};
use std::sync::Arc;

pub async fn get_document(Extension(state): Extension<State>, Path(document_id): Path<i32>) {}

#[derive(Deserialize, Debug)]
struct CreateDocument {
    title: String,
    content: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/*
pub async fn create_document(Extension(state): Extension<State>,
                             Json(payload): Json<CreateDocument>) -> Json<Document> {
    Document::create(
        &state.db_pool,

    )
}
 */

pub async fn list_documents(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<ResultList<Document>>> {
    Ok(Json(ResultList {
        results: Document::list(&state.db_pool).await?,
    }))
}
