use crate::data::models::content_field::ContentField;
use crate::error::ServerResult;
use crate::serializers::results::ResultList;
use crate::state::State;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentFieldSerializer {
    pub id: i32,
    pub name: Option<String>,
    pub code: String,
    pub system: bool,
    pub schema: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ContentField> for ContentFieldSerializer {
    fn from(value: ContentField) -> Self {
        Self {
            id: value.id,
            name: value.name,
            code: value.code,
            system: value.system,
            schema: value.schema,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn list_content_fields(
    Extension(state): Extension<Arc<State>>,
) -> ServerResult<Json<ResultList<ContentFieldSerializer>>> {
    let content_fields = ContentField::list(&state.db_pool)
        .await?
        .into_iter()
        .map(ContentFieldSerializer::from)
        .collect();
    Ok(Json(ResultList {
        results: content_fields,
    }))
}
