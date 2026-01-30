# Builder stage  
FROM rust:latest as builder

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
COPY --from=builder /app/target/release/orion-ml-fraud-agent /usr/local/bin/

# Create models directory and copy trained weights
RUN mkdir -p /app/models
COPY models/fraud_weights.json /app/models/fraud_weights.json

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
