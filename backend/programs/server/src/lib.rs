mod config;
mod constants;
mod data;
pub mod error;
mod routes;
mod services;
mod state;

use crate::config::Config;
use crate::data::connection::create_db_pool;
use crate::error::ServerResult;
use crate::routes::v1::health_check::health_check;
use crate::state::State;
use axum::{routing::get, Extension, Router};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub async fn run_server() {
    log::info!("Starting server");

    let config = Config::from_env().expect("Couldn't load config");

    let db_pool = create_db_pool(&config)
        .await
        .expect("Couldn't connect to database");

    let state = Arc::new(State { db_pool });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .layer(Extension(state));

    let addr = SocketAddr::from_str(&config.addr)
        .expect(&format!("Couldn't parse address from {}", config.addr));

    log::info!("Running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
