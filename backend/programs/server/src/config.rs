use crate::error::ServerResult;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String
}

impl Config {
    pub fn from_env() -> ServerResult<Self> {
        dotenv()?;
        let database_url: String = env::var("DATABASE_URL")?;
        let host: String = env::var("HOST")?;
        let port: String = env::var("PORT")?;
        Ok(
            Config {
                database_url,
                host,
                port
            }
        )
    }
}