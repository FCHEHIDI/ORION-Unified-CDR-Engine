mod config;
mod metrics;
mod routes;
mod service;

use axum::{Router, routing::get};
use config::Config;
use metrics_exporter_prometheus::PrometheusBuilder;
use service::KafkaConsumerService;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orion_validation=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting orion-validation service");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded - Server: {}:{}, Input topic: {}, Output topic: {}, Rejected topic: {}",
        config.server.host,
        config.server.port,
        config.kafka.input_topic,
        config.kafka.output_topic,
        config.kafka.rejected_topic,
    );

    // Initialize metrics
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()?;
    metrics::init_metrics();
    tracing::info!("Metrics initialized");

    // Create Kafka consumer service
    let kafka_service = KafkaConsumerService::new(&config.kafka)?;
    tracing::info!("Kafka consumer service initialized");

    // Spawn Kafka consumer
    tokio::spawn(async move {
        if let Err(e) = kafka_service.run().await {
            tracing::error!("Kafka consumer error: {}", e);
        }
    });

    // Build HTTP server
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/metrics", get({
            let handle = prometheus_handle.clone();
            move || routes::metrics(handle)
        }));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
