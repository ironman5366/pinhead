mod config;
mod constants;
mod data;
pub mod error;
mod middleware;
mod routes;
mod serializers;
mod services;
mod state;

use crate::config::Config;
use crate::data::connection::create_db_pool;
use crate::error::ServerResult;
use crate::middleware::auth_middleware::auth_middleware;
use crate::routes::v1::document::{create_document, list_documents};
use crate::routes::v1::health_check::health_check;
use crate::state::State;
use axum::http::Method;
use axum::middleware::from_fn as middleware_from_fn;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn run_server() {
    log::info!("Starting server");

    let config = Config::from_env().expect("Couldn't load config");

    let db_pool = create_db_pool(&config)
        .await
        .expect("Couldn't connect to database");

    let state = Arc::new(State { db_pool });

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_origin(Any)
        .allow_headers(Any);

    let auth_middleware = middleware_from_fn(auth_middleware);
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route(
            "/api/v1/documents/",
            get(list_documents).post(create_document),
        )
        .route_layer(auth_middleware)
        .layer(Extension(state))
        .layer(cors);

    let addr = SocketAddr::from_str(&config.addr)
        .expect(&format!("Couldn't parse address from {}", config.addr));

    log::info!("Running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
