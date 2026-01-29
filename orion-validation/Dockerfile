# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy workspace Cargo.toml
COPY Cargo.toml ./

# Copy the specific crate
COPY orion-validation ./orion-validation

# Build release
RUN cd orion-validation && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/orion-validation/target/release/orion-validation /usr/local/bin/

# Expose ports
EXPOSE 8082

# Set default environment
ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8082

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8082/health || exit 1

CMD ["orion-validation"]
