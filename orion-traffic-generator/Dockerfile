FROM rust:1.83 as builder

WORKDIR /app

# Install build dependencies for rdkafka
RUN apt-get update && apt-get install -y cmake build-essential && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /opt/orion
COPY --from=builder /app/target/release/orion-traffic-generator .
CMD ["./orion-traffic-generator"]
