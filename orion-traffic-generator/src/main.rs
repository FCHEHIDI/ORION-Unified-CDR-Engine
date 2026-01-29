mod config;
mod generator;
mod metrics;

use axum::{routing::get, Router};
use config::Config;
use generator::TrafficGenerator;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "traffic_generator=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting ORION Traffic Generator");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded - Rate: {} CDR/s, Burst: {}x, Fraud: {}%, Malformed: {}%, Errors: {}%",
        config.generation.rate_per_second,
        config.generation.burst_multiplier,
        config.generation.fraud_rate_percent,
        config.generation.malformed_rate_percent,
        config.simulation.error_rate_percent,
    );

    // Initialize metrics
    let prometheus_handle = PrometheusBuilder::new().install_recorder()?;
    metrics::init_metrics();
    tracing::info!("Metrics initialized");

    // Create traffic generator
    let generator = TrafficGenerator::new(config.clone())?;
    tracing::info!("Traffic generator initialized");

    // Spawn generator task
    tokio::spawn(async move {
        generator.run().await;
    });

    // Build HTTP server
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get({
            let handle = prometheus_handle.clone();
            move || async move { handle.render() }
        }));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("HTTP server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
