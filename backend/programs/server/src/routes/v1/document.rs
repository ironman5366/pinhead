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
pub struct CreateDocumentSerializer {
    pub title: String,
    pub content: Value
}


pub async fn create_document(Extension(state): Extension<Arc<State>>,
                             Json(payload): Json<CreateDocumentSerializer>) -> ServerResult<Json<Document>> {
    Ok(Json(Document::create(
        &state.db_pool,
            payload.title,
            payload.content
    ).await?))
}

pub async fn list_documents(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<ResultList<Document>>> {
    Ok(Json(ResultList {
        results: Document::list(&state.db_pool).await?,
    }))
}
