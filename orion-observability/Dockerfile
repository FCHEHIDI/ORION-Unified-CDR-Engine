# Builder stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy service files
COPY Cargo.toml ./
COPY src ./src

# Build the service
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/orion-observability /usr/local/bin/

# Non-root user for security
RUN useradd -m -u 1000 orion && \
    chown -R orion:orion /app
USER orion

# Environment variables (can be overridden)
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=9100
ENV HEALTH_CHECK_INTERVAL_SECS=30
ENV RUST_LOG=info

EXPOSE 9100

HEALTHCHECK --interval=10s --timeout=5s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:9100/health || exit 1

CMD ["orion-observability"]
