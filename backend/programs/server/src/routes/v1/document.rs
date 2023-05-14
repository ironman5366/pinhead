use crate::data::models::content_type::ContentType;
use crate::data::models::document::Document;
use crate::error::{ServerError, ServerResult};
use crate::serializers::document::DocumentSerializer;
use crate::serializers::results::ResultList;
use crate::state::State;
use axum::{extract::Path, Extension, Json};
use hyper::Server;
use serde::{Deserialize, Serialize};
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
        Document::create(
            &state.db_pool,
            payload.title,
            payload.content_type_id,
            payload.content,
        )
        .await?
        .into(),
    ))
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum DocumentByTypeResponse {
    SingleField(Option<DocumentSerializer>),
    CollectionField(Vec<DocumentSerializer>),
}

#[axum_macros::debug_handler]
pub async fn list_documents_by_type(
    Extension(state): Extension<Arc<State>>,
    Path(code): Path<String>,
) -> ServerResult<Json<DocumentByTypeResponse>> {
    // TODO: should just do this in one query
    let content_type = ContentType::get_by_code(&state.db_pool, code).await?;
    let documents = Document::list_by_content_type(&state.db_pool, content_type.id).await?;

    let doc_response: DocumentByTypeResponse;
    if content_type.collection {
        match documents.as_slice() {
            [] => doc_response = DocumentByTypeResponse::SingleField(None),
            [document] => doc_response = DocumentByTypeResponse::SingleField(Some(document.into())),
            _ => return Err(ServerError::MultipleDocumentsReturnedError),
        }
    } else {
        doc_response = DocumentByTypeResponse::CollectionField(
            documents
                .into_iter()
                .map(DocumentSerializer::from)
                .collect(),
        )
    }

    Ok(Json(doc_response))
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
