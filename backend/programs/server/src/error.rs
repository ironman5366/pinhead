use axum::http;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
    #[error(transparent)]
    DotEnvError(#[from] dotenv::Error),
    #[error(transparent)]
    ArgonError(#[from] argon2::Error)
}

pub type ServerResult<T> = Result<T, ServerError>;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        // TODO: make this more robust, maybe keep this for DEBUG mode?
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let message = json!({
            "error": status.to_string(),
            "message": self.to_string()
        });

        let body = match serde_json::to_string(&message) {
            Ok(val) => val,
            // Bad place to be!
            Err(_) => {
                "{\"error\": 500, \"message\": \"Internal Error: Failed to parse\"}".to_string()
            }
        };

        (status, body).into_response()
    }
}
