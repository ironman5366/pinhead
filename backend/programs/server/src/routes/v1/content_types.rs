use crate::data::models::content_type::ContentType;
use crate::error::ServerResult;
use crate::serializers::results::ResultList;
use crate::state::State;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentTypeSerializer {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ContentType> for ContentTypeSerializer {
    fn from(value: ContentType) -> Self {
        ContentTypeSerializer {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn list_content_types(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<ResultList<ContentTypeSerializer>>> {
    let content_types = ContentType::list(&state.db_pool)
        .await?
        .into_iter()
        .map(ContentTypeSerializer::from)
        .collect();

    Ok(Json(ResultList {
        results: content_types,
    }))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateContentTypeSerializer {
    pub name: String,
    pub field_ids: Vec<i32>,
}

#[axum_macros::debug_handler]
pub async fn create_content_type(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<CreateContentTypeSerializer>,
) -> ServerResult<Json<ContentTypeSerializer>> {
    Ok(Json(
        ContentType::create(&state.db_pool, payload.name, payload.field_ids)
            .await?
            .into(),
    ))
}
