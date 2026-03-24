# --- 第一階段：Rust 編譯 ---
FROM rust:slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

# --- 第二階段：最終運行環境 ---
FROM debian:bookworm-slim

WORKDIR /app

# 合併安裝與清理，減少鏡像層
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    libgl1-mesa-glx \
    libglib2.0-0 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# 優化重點：安裝 CPU 版 PyTorch 並移除 pip 緩存
RUN pip3 install --no-cache-dir ultralytics \
    --extra-index-url https://download.pytorch.org/whl/cpu \
    --break-system-packages \
    && rm -rf /root/.cache/pip

# 複製編譯好的 Rust 執行檔
COPY --from=builder /app/target/release/healthy-diet-api-server /app/server

# 預下載模型（這部分無法省，但可以確保模型封裝在鏡像中）
RUN python3 -c "from ultralytics import YOLO; YOLO('yolo11n.pt')"

RUN mkdir -p /app/uploads

EXPOSE 3000
CMD ["./server"]