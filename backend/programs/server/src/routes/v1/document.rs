use crate::data::models::document::Document;
use crate::error::ServerResult;
use crate::serializers::document::DocumentSerializer;
use crate::serializers::results::ResultList;
use crate::state::State;
use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentSerializer {
    pub title: String,
    pub content: Value,
    pub content_type_id: i32,
}

#[axum_macros::debug_handler]
pub async fn create_document(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<CreateDocumentSerializer>,
) -> ServerResult<Json<DocumentSerializer>> {
    Ok(Json(
        Document::create(&state.db_pool, payload.title, payload.content)
            .await?
            .into(),
    ))
}

#[axum_macros::debug_handler]
pub async fn list_documents(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<ResultList<DocumentSerializer>>> {
    // List all the documents and turn them into document serializers
    let documents = Document::list(&state.db_pool)
        .await?
        .into_iter()
        .map(DocumentSerializer::from)
        .collect();
    Ok(Json(ResultList { results: documents }))
}
