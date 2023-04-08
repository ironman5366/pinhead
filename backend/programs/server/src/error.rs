use axum::http;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hyper::header::ToStrError;
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
    ArgonError(#[from] argon2::Error),
    #[error("{0}")]
    BadRequestError(String),
    #[error("Unauthorized")]
    UnauthorizedError,
    #[error(transparent)]
    HeaderToStrError(#[from] ToStrError),
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    StatusError(String, StatusCode),
    #[error("Malformed token")]
    MalformedTokenError,
    #[error("Invalid Token")]
    InvalidTokenError,
    #[error("Expired Token")]
    ExpiredTokenError,
}

pub type ServerResult<T> = Result<T, ServerError>;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::StatusError(_, code) => code,
            Self::BadRequestError(_) => StatusCode::BAD_REQUEST,
            Self::MalformedTokenError => StatusCode::BAD_REQUEST,
            Self::UnauthorizedError => StatusCode::UNAUTHORIZED,
            Self::InvalidTokenError => StatusCode::UNAUTHORIZED,
            Self::ExpiredTokenError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

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
