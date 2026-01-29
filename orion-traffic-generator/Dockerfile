FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /opt/orion
COPY --from=builder /app/target/release/orion-traffic-generator .
CMD ["./orion-traffic-generator"]
