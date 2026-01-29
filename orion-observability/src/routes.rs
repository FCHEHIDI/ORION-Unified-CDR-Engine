use crate::health::{HealthChecker, PipelineHealth, ServiceHealth};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;
use std::sync::Arc;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub health_checker: Arc<HealthChecker>,
}

/// Health check response for this service
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

/// Self health check endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "orion-observability".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Metrics endpoint (Prometheus format)
pub async fn metrics(handle: PrometheusHandle) -> String {
    handle.render()
}

/// Get aggregated pipeline health - GET /pipeline/health
pub async fn pipeline_health(
    State(state): State<AppState>,
) -> Result<Json<PipelineHealth>, AppError> {
    let health = state.health_checker.get_pipeline_health().await;
    Ok(Json(health))
}

/// Get individual service health - GET /services/:name/health
pub async fn service_health(
    State(state): State<AppState>,
    Path(service_name): Path<String>,
) -> Result<Json<ServiceHealth>, AppError> {
    match state.health_checker.get_service_health(&service_name).await {
        Some(health) => Ok(Json(health)),
        None => Err(AppError::NotFound(format!(
            "Service '{}' not found",
            service_name
        ))),
    }
}

/// Application error type
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        };
        
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

/// Error response format
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_structure() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            service: "orion-observability".to_string(),
            version: "0.1.0".to_string(),
        };
        
        assert_eq!(response.status, "healthy");
        assert_eq!(response.service, "orion-observability");
    }
}
