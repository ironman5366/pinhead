use crate::middleware::auth_middleware::auth_middleware;
use crate::routes::v1::document::{create_document, list_documents};
use crate::routes::v1::health_check::health_check;
use crate::routes::v1::user::{current, login, register};
use axum::middleware::from_fn as middleware_from_fn;
use axum::routing::{get, post};
use axum::Router;

fn user_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);

    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/current", get(current).route_layer(auth_fn))
}

fn document_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);
    Router::new()
        .route("/", get(list_documents))
        .route("/", post(create_document))
        // Apply auth middleware to all document routes
        .layer(auth_fn)
}

pub fn router() -> Router<()> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/documents", document_routes())
        .route("/health_check", get(health_check))
}
