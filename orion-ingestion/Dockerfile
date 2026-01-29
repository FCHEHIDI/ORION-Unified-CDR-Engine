# Build stage
FROM rust:1.83 as builder

WORKDIR /app

# Install build dependencies for rdkafka
RUN apt-get update && apt-get install -y cmake build-essential && rm -rf /var/lib/apt/lists/*

# Copy service files
COPY Cargo.toml ./Cargo.toml
COPY src ./src

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y cmake build-essential \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/orion-ingestion /usr/local/bin/

# Expose ports
EXPOSE 8081

# Set default environment
ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8081

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8081/health || exit 1

CMD ["orion-ingestion"]
