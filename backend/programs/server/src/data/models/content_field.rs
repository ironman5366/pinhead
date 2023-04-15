use jsonschema::JSONSchema;

#[derive(sqlx::FromRow, Debug)]
pub struct ContentField {
    pub id: i32,
    pub name: Option<String>,
    // Code is unique across fields
    pub code: String,
    // Whether this is a system type or user created
    pub system: bool,
    pub schema: JSONSchema,
}
