mod config;
mod metrics;
mod routes;
mod service;

use axum::{
    routing::get,
    Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tower_http::trace::TraceLayer;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::service::{KafkaConsumerService, ProcessedCDR};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orion_ingestion=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    info!("🚀 Starting ORION Ingestion Service...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded: {:?}", config);

    // Initialize metrics
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()?;
    metrics::init_metrics();
    info!("✅ Metrics initialized");

    // Create output channel for processed CDR
    let (output_tx, mut output_rx) = mpsc::channel::<ProcessedCDR>(1000);

    // Initialize Kafka consumer
    let kafka_service = KafkaConsumerService::new(&config.kafka, output_tx)?;
    info!("✅ Kafka consumer initialized");

    // Spawn Kafka consumer task
    let kafka_handle = tokio::spawn(async move {
        if let Err(e) = kafka_service.run().await {
            error!("Kafka consumer error: {:?}", e);
        }
    });

    // Spawn output handler (for now, just log)
    // In production, this would publish to the next Kafka topic
    let output_handle = tokio::spawn(async move {
        while let Some(processed_cdr) = output_rx.recv().await {
            info!(
                "📤 Processed CDR ready for validation - Country: {}, Topic: {}",
                processed_cdr.country,
                processed_cdr.source_topic
            );
            // TODO: Publish to Kafka topic "cdr.validated"
        }
    });

    // Build HTTP server with routes
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/metrics", get(move || async move {
            prometheus_handle.render()
        }))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()?,
        config.server.port,
    ));

    info!("🌐 HTTP server listening on {}", addr);
    info!("  - Health: http://{}/health", addr);
    info!("  - Metrics: http://{}/metrics", addr);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("HTTP server error: {:?}", e);
        }
    });

    info!("✅ ORION Ingestion Service fully operational");

    // Wait for all tasks
    tokio::select! {
        _ = kafka_handle => error!("Kafka consumer task exited"),
        _ = output_handle => error!("Output handler task exited"),
        _ = server_handle => error!("HTTP server task exited"),
    }

    Ok(())
}

