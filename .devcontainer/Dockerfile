# Builder stage
FROM mcr.microsoft.com/devcontainers/rust:latest AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    bash \
    curl \
    gnupg \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    binaryen && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /work
COPY . .

# Install cargo-leptos
RUN rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src && \
    cargo build --target wasm32-unknown-unknown || true
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

RUN cargo leptos build

# Runtime stage
FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    glibc-utils \
    libpq5 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /root/.devcontainer && chmod -R 777 /root/.devcontainer

WORKDIR /app

COPY --from=builder /work/target/release/my-webpage .
COPY --from=builder /work/target/site ./site
RUN chmod -R 777 ./site
RUN chmod +x ./my-webpage

RUN addgroup --gid 1000 appgroup && adduser --uid 1000 --gid 1000 --disabled-password --gecos '' appuser
USER appuser

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ROOT="./site"

EXPOSE 8080
CMD ["/bin/sh"]
#CMD ["/app/my-webpage"]
