use crate::data::data_field::DataField;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentField {
    pub name: String,
    pub code: String,
    pub field_type: String,
}

#[derive(FromRow, Debug)]
pub struct ContentType {
    pub id: i32,
    pub name: String,
    pub fields: Vec<ContentField>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
