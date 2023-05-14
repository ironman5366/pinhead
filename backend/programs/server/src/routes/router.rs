use crate::middleware::auth_middleware::auth_middleware;
use crate::routes::v1::content_field::{code_available, create_content_field, list_content_fields};
use crate::routes::v1::content_types::list_content_types;
use crate::routes::v1::document::{create_document, list_documents, list_documents_by_type};
use crate::routes::v1::health_check::health_check;
use crate::routes::v1::user::{current, login, register};
use axum::middleware::from_fn as middleware_from_fn;
use axum::routing::{get, post};
use axum::Router;

fn user_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);

    Router::new()
        .route("/login/", post(login))
        .route("/register/", post(register))
        .route("/current/", get(current).route_layer(auth_fn))
}

fn document_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);
    Router::new()
        .route("/", get(list_documents))
        .route("/:type", get(list_documents_by_type))
        .route("/", post(create_document))
        // Apply auth middleware to all document routes
        .layer(auth_fn)
}

fn content_field_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);
    Router::new()
        .route("/", get(list_content_fields))
        .route("/", post(create_content_field))
        .route("/code_available/:code/", get(code_available))
        .layer(auth_fn)
}

fn content_type_routes() -> Router<()> {
    let auth_fn = middleware_from_fn(auth_middleware);
    Router::new()
        .route("/", get(list_content_types))
        .layer(auth_fn)
}

pub fn router() -> Router<()> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/documents", document_routes())
        .nest("/content_types", content_type_routes())
        .nest("/content_fields", content_field_routes())
        .route("/health_check/", get(health_check))
}
