# Builder stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace Cargo files
COPY Cargo.toml ./

# Copy service source
COPY orion-ml-fraud-agent ./orion-ml-fraud-agent

# Build the service
WORKDIR /app/orion-ml-fraud-agent
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/orion-ml-fraud-agent/target/release/orion-ml-fraud-agent /usr/local/bin/

# Create models directory
RUN mkdir -p /app/models

# Non-root user for security
RUN useradd -m -u 1000 orion && \
    chown -R orion:orion /app
USER orion

# Environment variables (can be overridden)
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8090
ENV MODEL_PATH=/app/models/fraud_detector.onnx
ENV FRAUD_THRESHOLD=0.5
ENV MODEL_BATCH_SIZE=32
ENV ENABLE_CUDA=false
ENV RUST_LOG=info

EXPOSE 8090

HEALTHCHECK --interval=10s --timeout=5s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8090/health || exit 1

CMD ["orion-ml-fraud-agent"]
