FROM rust:slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    libgl1-mesa-glx \
    libglib2.0-0 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install ultralytics --break-system-packages

WORKDIR /app

COPY --from=builder /app/target/release/healthy-diet-api-server /app/server

RUN python3 -c "from ultralytics import YOLO; YOLO('yolo11n.pt')"

RUN mkdir -p /app/uploads

EXPOSE 3000
CMD ["./server"]
