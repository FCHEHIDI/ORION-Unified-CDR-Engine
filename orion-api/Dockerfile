FROM rust:1.75-slim-bullseye AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 orion

WORKDIR /app

COPY --from=builder /app/target/release/orion-api .

RUN chown -R orion:orion /app

USER orion

EXPOSE 8080

CMD ["./orion-api"]
