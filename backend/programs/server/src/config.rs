use crate::error::ServerResult;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub addr: String,
}

impl Config {
    pub fn from_env() -> ServerResult<Self> {
        dotenv()?;
        let database_url: String = env::var("DATABASE_URL")?;
        let addr: String = env::var("ADDR")?;
        Ok(Config { database_url, addr })
    }
}
