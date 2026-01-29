use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::service::ArchiveService;

pub fn create_router(service: Arc<ArchiveService>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/stats", get(get_stats))
        .with_state(service)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

async fn get_stats(State(service): State<Arc<ArchiveService>>) -> impl IntoResponse {
    let stats = service.get_stats().await;
    (StatusCode::OK, Json(stats))
}
