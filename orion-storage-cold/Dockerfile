FROM rustlang/rust:nightly-bullseye-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src target/release/deps/orion_storage_cold*

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 orion

WORKDIR /app

RUN mkdir -p /tmp/orion-parquet && chown orion:orion /tmp/orion-parquet

COPY --from=builder /app/target/release/orion-storage-cold .

RUN chown -R orion:orion /app

USER orion

EXPOSE 9400

CMD ["./orion-storage-cold"]
