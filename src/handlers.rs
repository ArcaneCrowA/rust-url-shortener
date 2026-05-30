use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{models::RequestDTO, services::Service};

pub async fn shorten(
    Extension(services): Extension<Arc<Service>>,
    Json(payload): Json<RequestDTO>,
) -> impl IntoResponse {
    match services.create_url(&payload.url).await {
        Ok(link) => (StatusCode::OK, Json(link)).into_response(),
        Err(err_msg) => (StatusCode::BAD_REQUEST, Json(err_msg)).into_response(),
    }
}

pub async fn open(
    Extension(services): Extension<Arc<Service>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    match services.get_url(&code).await {
        Ok(url) => Redirect::to(&url).into_response(),
        Err(err_msg) => (StatusCode::NOT_FOUND, Json(err_msg)).into_response(),
    }
}
