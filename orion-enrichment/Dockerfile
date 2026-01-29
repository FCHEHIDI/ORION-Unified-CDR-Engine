# Build stage
FROM rust:1.83 AS builder

WORKDIR /app

# Install build dependencies for rdkafka and openssl
RUN apt-get update && apt-get install -y cmake build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*


# Copy manifests
COPY Cargo.toml ./Cargo.toml

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release

# Remove dummy files and binary
RUN rm -rf src target/release/deps/orion_enrichment*

# Copy real source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies for librdkafka and openssl
RUN apt-get update && apt-get install -y cmake build-essential pkg-config libssl-dev \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/orion-enrichment .

# Expose HTTP port
EXPOSE 8084

CMD ["./orion-enrichment"]
