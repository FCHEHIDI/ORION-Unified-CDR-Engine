# create_orion_dockerfiles.ps1
# Génère un Dockerfile standard pour chaque microservice ORION

$services = @(
    "orion-ingestion",
    "orion-validation",
    "orion-normalization",
    "orion-enrichment",
    "orion-ml-fraud-agent",
    "orion-storage-hot",
    "orion-storage-cold",
    "orion-api",
    "orion-observability"
)

foreach ($svc in $services) {
    $dockerfilePath = "./$svc/Dockerfile"

    $content = @"
# ============================
# ORION Dockerfile for $svc
# ============================

# ---- Build stage ----
FROM rust:1.75 as builder

WORKDIR /app

# Copy workspace
COPY ../../Cargo.toml ../../Cargo.lock ./
COPY ../../orion-* ../

# Build only this service
WORKDIR /app/$svc
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:stable-slim

# Create non-root user
RUN useradd -m orion

WORKDIR /opt/orion

# Copy binary
COPY --from=builder /app/$svc/target/release/$svc /opt/orion/$svc

# Permissions
RUN chown -R orion:orion /opt/orion

USER orion

CMD ["./$svc"]
"@

    Set-Content -Path $dockerfilePath -Value $content
}

Write-Host "Dockerfiles ORION générés avec succès."
