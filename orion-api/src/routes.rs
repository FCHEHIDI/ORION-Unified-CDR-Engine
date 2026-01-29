use crate::repository::{Cdr, CdrRepository};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<CdrRepository>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "orion-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn metrics(handle: PrometheusHandle) -> String {
    handle.render()
}

pub async fn get_cdr_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Cdr>, AppError> {
    match state.repository.get_by_id(&id).await? {
        Some(cdr) => Ok(Json(cdr)),
        None => Err(AppError::NotFound(format!("CDR {} not found", id))),
    }
}

#[derive(Deserialize)]
pub struct SearchParams {
    country_code: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    #[serde(default = "default_limit")]
    limit: i32,
}

fn default_limit() -> i32 {
    100
}

pub async fn search_cdr(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Cdr>>, AppError> {
    let cdrs = state
        .repository
        .search(
            params.country_code,
            params.start_time,
            params.end_time,
            params.limit,
        )
        .await?;
    Ok(Json(cdrs))
}

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

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
