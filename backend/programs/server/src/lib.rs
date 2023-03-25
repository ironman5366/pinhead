mod config;
mod data;
pub mod error;
mod services;
mod constants;

use axum::{routing::get, Router};
use crate::config::Config;
use crate::error::ServerResult;

pub async fn run_server() {
    let config = Config::from_env()
        .expect("Couldn't load config");
}
