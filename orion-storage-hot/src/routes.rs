use axum::{http::StatusCode, response::IntoResponse, Json};
use metrics_exporter_prometheus::PrometheusHandle;
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "orion-storage-hot"
        })),
    )
}

pub async fn metrics(handle: PrometheusHandle) -> impl IntoResponse {
    handle.render()
}
