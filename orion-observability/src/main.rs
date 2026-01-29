mod config;
mod health;
mod metrics;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use health::HealthChecker;
use metrics_exporter_prometheus::PrometheusBuilder;
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
                .unwrap_or_else(|_| "orion_observability=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting orion-observability service");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded - Server: {}:{}, Health check interval: {}s",
        config.server.host,
        config.server.port,
        config.services.health_check_interval_secs,
    );

    // Initialize metrics
    let prometheus_handle = PrometheusBuilder::new().install_recorder()?;
    metrics::init_metrics();
    tracing::info!("Metrics initialized");

    // Create health checker
    let health_checker = Arc::new(HealthChecker::new(config.services.clone()));
    tracing::info!("Health checker initialized");

    // Start background health monitoring
    let health_checker_clone = health_checker.clone();
    tokio::spawn(async move {
        health_checker_clone.start_monitoring().await;
    });

    // Create application state
    let state = AppState { health_checker };

    // Build HTTP server
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/metrics", get({
            let handle = prometheus_handle.clone();
            move || routes::metrics(handle)
        }))
        .route("/pipeline/health", get(routes::pipeline_health))
        .route("/services/:name/health", get(routes::service_health))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
