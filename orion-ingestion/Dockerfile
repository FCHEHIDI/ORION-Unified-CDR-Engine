# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy workspace Cargo.toml
COPY Cargo.toml ./

# Copy the specific crate
COPY orion-ingestion ./orion-ingestion

# Build release
RUN cd orion-ingestion && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/orion-ingestion/target/release/orion-ingestion /usr/local/bin/

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
