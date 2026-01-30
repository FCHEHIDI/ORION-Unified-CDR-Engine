mod config;
mod features;
mod metrics;
mod model;
mod routes;
mod simple_ml;

use axum::{routing::{get, post}, Router};
use config::Config;
use metrics_exporter_prometheus::PrometheusBuilder;
use model::FraudDetector;
use routes::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orion_ml_fraud_agent=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting orion-ml-fraud-agent service");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded - Server: {}:{}, Model: {}, Threshold: {}, CUDA: {}",
        config.server.host,
        config.server.port,
        config.model.path,
        config.model.threshold,
        config.model.enable_cuda,
    );

    // Initialize metrics
    let prometheus_handle = PrometheusBuilder::new().install_recorder()?;
    metrics::init_metrics();
    tracing::info!("Metrics initialized");

    // Load ML model
    tracing::info!("Loading fraud detection model...");
    let detector = FraudDetector::new(&config.model).await?;
    tracing::info!("Fraud detection model loaded successfully");

    // Create application state
    let state = AppState {
        detector: Arc::new(detector),
    };

    // Build HTTP server
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/metrics", get({
            let handle = prometheus_handle.clone();
            move || routes::metrics(handle)
        }))
        .route("/predict", post(routes::predict))
        .route("/predict/batch", post(routes::predict_batch))
        .route("/model/info", get(routes::model_info))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
