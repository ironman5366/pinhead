use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
    #[error(transparent)]
    DotEnvError(#[from] dotenv::Error),
}

pub type ServerResult<T> = Result<T, ServerError>;
