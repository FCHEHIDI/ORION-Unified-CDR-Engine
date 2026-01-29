use axum::{response::IntoResponse, http::StatusCode};
use metrics_exporter_prometheus::PrometheusHandle;

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn metrics(handle: PrometheusHandle) -> impl IntoResponse {
    handle.render()
}
