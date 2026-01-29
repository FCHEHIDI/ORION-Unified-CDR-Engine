use crate::features::{FraudFeatures, FraudPrediction};
use crate::model::{FraudDetector, ModelInfo};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub detector: Arc<FraudDetector>,
}

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

/// Health check endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "orion-ml-fraud-agent".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Metrics endpoint (Prometheus format)
pub async fn metrics(handle: PrometheusHandle) -> String {
    handle.render()
}

/// Prediction request for single CDR
#[derive(Debug, Deserialize)]
pub struct PredictRequest {
    pub features: FraudFeatures,
}

/// Batch prediction request
#[derive(Debug, Deserialize)]
pub struct BatchPredictRequest {
    pub features_batch: Vec<FraudFeatures>,
}

/// Prediction endpoint - POST /predict
pub async fn predict(
    State(state): State<AppState>,
    Json(request): Json<PredictRequest>,
) -> Result<Json<FraudPrediction>, AppError> {
    let prediction = state.detector.predict(&request.features).await?;
    Ok(Json(prediction))
}

/// Batch prediction endpoint - POST /predict/batch
pub async fn predict_batch(
    State(state): State<AppState>,
    Json(request): Json<BatchPredictRequest>,
) -> Result<Json<Vec<FraudPrediction>>, AppError> {
    let predictions = state.detector.predict_batch(&request.features_batch).await?;
    Ok(Json(predictions))
}

/// Model info endpoint - GET /model/info
pub async fn model_info(
    State(state): State<AppState>,
) -> Result<Json<ModelInfo>, AppError> {
    let info = state.detector.model_info().await;
    Ok(Json(info))
}

/// Application error type
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Request error: {:?}", self.0);
        
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
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
            service: "orion-ml-fraud-agent".to_string(),
            version: "0.1.0".to_string(),
        };
        
        assert_eq!(response.status, "healthy");
        assert_eq!(response.service, "orion-ml-fraud-agent");
    }
}
