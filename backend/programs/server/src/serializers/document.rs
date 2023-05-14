use crate::data::models::document::Document;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSerializer {
    pub id: i32,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<Document> for DocumentSerializer {
    fn from(document: Document) -> Self {
        Self {
            id: document.id,
            title: document.title,
            created_at: document.created_at,
            updated_at: document.updated_at,
        }
    }
}

impl From<&Document> for DocumentSerializer {
    fn from(document: &Document) -> Self {
        Self {
            id: document.id,
            title: document.title.clone(),
            created_at: document.created_at,
            updated_at: document.updated_at,
        }
    }
}
