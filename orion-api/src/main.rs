mod config;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use metrics_exporter_prometheus::PrometheusBuilder;
use repository::CdrRepository;
use routes::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orion_api=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting orion-api service");

    let config = Config::from_env()?;
    tracing::info!("Configuration loaded - Server: {}:{}", config.server.host, config.server.port);

    let prometheus_handle = PrometheusBuilder::new().install_recorder()?;
    tracing::info!("Metrics initialized");

    let repository = Arc::new(CdrRepository::new(&config.scylla).await?);
    tracing::info!("ScyllaDB repository initialized");

    let state = AppState { repository };

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/metrics", get({
            let handle = prometheus_handle.clone();
            move || routes::metrics(handle)
        }))
        .route("/cdr/:id", get(routes::get_cdr_by_id))
        .route("/cdr/search", get(routes::search_cdr))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
