FROM rust:slim-bookworm as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app

ENV SQLX_OFFLINE=true

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/healthy-diet-api-server /app/server

EXPOSE 3000

CMD ["./server"]