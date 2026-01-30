# Build stage
FROM rust:1.83 AS builder

WORKDIR /app

# Install build dependencies for rdkafka
RUN apt-get update && apt-get install -y cmake build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*


# Copy manifests
COPY Cargo.toml ./Cargo.toml

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release

# Remove dummy files and binary
RUN rm -rf src target/release/deps/orion_storage_hot*

# Copy real source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/orion-storage-hot .

# Expose HTTP port
EXPOSE 8085

CMD ["./orion-storage-hot"]
