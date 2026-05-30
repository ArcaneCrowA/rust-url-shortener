use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{get, post},
};

use crate::{
    handlers::{open, shorten},
    repositories::RedisRepository,
    services::Service,
};

mod errors;
mod handlers;
mod models;
mod repositories;
mod services;

#[tokio::main]
async fn main() {
    let repo = Arc::new(RedisRepository::new());
    let service = Arc::new(Service::new(repo));
    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/{code}", get(open))
        .layer(Extension(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
