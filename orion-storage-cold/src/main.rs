mod config;
mod metrics;
mod routes;
mod schema;
mod service;
mod uploader;
mod writer;

use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;

use config::Config;
use metrics::Metrics;
use service::ArchiveService;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let metrics = Arc::new(Metrics::new());

    info!("Starting ORION Cold Storage Service");
    info!("S3 endpoint: {}", config.s3.endpoint);
    info!("S3 bucket: {}", config.s3.bucket);
    info!("Kafka topic: {}", config.kafka.topic);

    let service = Arc::new(
        ArchiveService::new(&config, metrics.clone())
            .await
            .expect("Failed to initialize archive service"),
    );

    let app = routes::create_router(service.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server");

    info!("HTTP server listening on {}", addr);

    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("Server error");
    });

    let consumer_handle = tokio::spawn(async move {
        if let Err(e) = service.start_consumer(&config).await {
            error!("Consumer error: {}", e);
        }
    });

    tokio::select! {
        _ = server_handle => info!("Server stopped"),
        _ = consumer_handle => info!("Consumer stopped"),
    }

    Ok(())
}

