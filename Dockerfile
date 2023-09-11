FROM rust:1.71.1-slim-bookworm

RUN apt update && apt upgrade

RUN apt-get install -y --no-install-recommends \
    libgnutls30 \
    libssl3 \
    libsystemd0 \
    libssl-dev \
    pkg-config \
    libudev1 \
    tar \
    ca-certificates \
    wget \
    git

RUN export RUST_BACKTRACE=full \
 export RUST_LOG=debug

RUN cargo install cargo-watch

WORKDIR /app
COPY . .
