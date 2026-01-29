use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

/// Health check endpoint
/// Returns 200 OK with "OK" body
pub async fn health() -> Response {
    (StatusCode::OK, "OK").into_response()
}

/// Metrics endpoint - handled by metrics-exporter-prometheus
/// This is just a placeholder, the actual metrics are served by the exporter
pub async fn metrics_handler() -> Response {
    // This should not be called as metrics are handled by the exporter
    // But we keep it here for completeness
    (StatusCode::OK, "Metrics endpoint").into_response()
}
