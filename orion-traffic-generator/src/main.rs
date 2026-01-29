mod metrics;
mod generator;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    metrics::init_metrics();

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics::export_metrics));

    tokio::spawn(async {
        generator::run().await;
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 9200));
    println!("Traffic Generator running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
