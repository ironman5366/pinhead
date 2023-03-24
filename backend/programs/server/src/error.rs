use thiserror::Error;

#[derive(Error, Debug, Display)]
pub struct ServerError {
}

pub type ServerResult<T> = Result<T, ServerError>;
