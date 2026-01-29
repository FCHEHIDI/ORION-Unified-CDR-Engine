# Build stage
FROM rust:1.75 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release

# Remove dummy files
RUN rm -rf src

# Copy real source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/orion-enrichment .

# Expose HTTP port
EXPOSE 8084

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "sh", "-c", "wget --no-verbose --tries=1 --spider http://localhost:8084/health || exit 1" ]

CMD ["./orion-enrichment"]
