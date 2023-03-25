use crate::config::Config;
use crate::constants::MAX_DB_CONNECTIONS;
use crate::error::ServerResult;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn create_db_pool(config: &Config) -> ServerResult<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(config.database_url.as_str())
        .await?;
    Ok(pool)
}
