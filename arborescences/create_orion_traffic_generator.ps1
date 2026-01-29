# create_orion_traffic_generator.ps1
# Génère le microservice Rust orion-traffic-generator (squelette complet)

$root = "./orion-traffic-generator"
$src = "$root/src"

Write-Host "=== Création du microservice orion-traffic-generator ===" -ForegroundColor Cyan

# Arborescence
New-Item -ItemType Directory -Force -Path $root | Out-Null
New-Item -ItemType Directory -Force -Path $src | Out-Null

# Cargo.toml
$cargo = @"
[package]
name = "orion-traffic-generator"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
metrics = "0.21"
metrics-exporter-prometheus = "0.13"
rdkafka = { version = "0.36", features = ["tokio"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rand = "0.8"
"@
Set-Content "$root/Cargo.toml" $cargo

# main.rs
$main = @"
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
"@
Set-Content "$src/main.rs" $main

# metrics.rs
$metrics = @"
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static PROM_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn init_metrics() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install().expect("failed to install Prometheus recorder");
    PROM_HANDLE.set(handle).unwrap();
}

pub async fn export_metrics() -> String {
    PROM_HANDLE.get().unwrap().render()
}
"@
Set-Content "$src/metrics.rs" $metrics

# generator.rs
$generator = @"
use tokio::time::{sleep, Duration};

pub async fn run() {
    loop {
        // Placeholder : génération sci-fi désactivée pour l'instant
        sleep(Duration::from_secs(5)).await;
    }
}
"@
Set-Content "$src/generator.rs" $generator

# Dockerfile
$docker = @"
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /opt/orion
COPY --from=builder /app/target/release/orion-traffic-generator .
CMD ["./orion-traffic-generator"]
"@
Set-Content "$root/Dockerfile" $docker

# Ajout au workspace Cargo
$workspaceFile = "./Cargo.toml"
if (Test-Path $workspaceFile) {
    $content = Get-Content $workspaceFile
    if ($content -notcontains '    "orion-traffic-generator"') {
        Write-Host "Ajout du service au workspace Cargo..."
        (Get-Content $workspaceFile) -replace '

\[workspace\]

\s*members\s*=\s*

\[', '[workspace]
members = [
    "orion-traffic-generator",' | Set-Content $workspaceFile
    }
}

Write-Host "=== Microservice orion-traffic-generator généré avec succès ===" -ForegroundColor Green
