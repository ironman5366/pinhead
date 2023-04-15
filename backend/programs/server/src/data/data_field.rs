use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DataField {
    Text(String),
    JSON(Value),
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    Boolean(bool),
    Markdown(String),
    Tiptap(Value),
}
