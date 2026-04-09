# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0
ARG APP_NAME=healthy-diet-api-server

# ── Build stage ──────────────────────────────────────────────
FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y \
    clang lld musl-dev git \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

RUN --mount=type=bind,source=healthy-diet-api/src,target=src \
    --mount=type=bind,source=healthy-diet-api/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=healthy-diet-api/Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=healthy-diet-api/.sqlx,target=.sqlx \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

COPY ./healthy-diet-api/AIPrompt.json /AIPrompt.json

# ── Final stage ───────────────────────────────────────────────
FROM nvidia/cuda:13.1.1-cudnn-runtime-ubuntu24.04 AS final
ARG UID=10001
WORKDIR /app

#RUN adduser \
#    --disabled-password \
#    --gecos "" \
#    --home "/nonexistent" \
#    --shell "/sbin/nologin" \
#    --no-create-home \
#    --uid "${UID}" \
#    appuser

RUN apt-get update && apt-get install -y \
    ca-certificates \
    python3 \
    python3-pip \
    libgl1 \
    libglx-mesa0 \
    libglib2.0-0 \
    && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

RUN pip3 install --no-cache-dir --break-system-packages \
    torch \
    torchvision \
    ultralytics==8.3.193 \
    opencv-python-headless==4.12.0.88 \
    numpy==2.2.6


#USER appuser
COPY --from=build /bin/server /bin/
#COPY healthy-diet-yolo/ /app/yolo_scripts/
RUN mkdir -p /root/.config/Ultralytics && chmod -R 700 /root/.config/Ultralytics
RUN mkdir -p /app/yolo_scripts
RUN mkdir -p /app/uploads

EXPOSE 3000
CMD ["bash", "-c", "sleep 2 && /bin/server"]
