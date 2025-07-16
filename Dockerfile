FROM docker.io/rustlang/rust:nightly-alpine as toolchain

RUN apk add --no-cache musl-dev git curl bash

# Install cargo-binstall (prebuilt binary)
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Use binstall to get prebuilt cargo-leptos
RUN cargo binstall cargo-leptos --locked --no-confirm

# Stage 0: Base image with dependencies (for dev/build stages)
FROM docker.io/rustlang/rust:nightly-alpine as base
RUN apk add --no-cache musl-dev openssl-dev npm bash shadow sudo git fish perl make openssh sccache && \
    npm i -g pnpm && \
    adduser -D vscode -s /usr/bin/fish && \
    echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
    rustup target add wasm32-unknown-unknown && \
    mkdir -p /app && chown vscode:vscode /app
ENV RUSTC_WRAPPER=sccache
WORKDIR /app
USER vscode
COPY --from=toolchain /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/

# Stage 1: Planner with prebuilt cargo-chef (fast)
FROM docker.io/cargo-chef/muslrust:nightly as planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cacher with prebuilt cargo-chef
FROM docker.io/cargo-chef/muslrust:nightly as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Builder
FROM base as builder
COPY --chown=vscode:vscode . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo leptos build --release

# Stage 4: Dev environment (hot reload)
FROM base as dev
RUN cargo install leptosfmt
COPY --chown=vscode:vscode . .
ENV LEPTOS_ENV=development \
    RUST_LOG=info
EXPOSE 8080
CMD ["cargo", "leptos", "watch", "--hot-reload"]

# Stage 5: Runtime
FROM alpine:latest as runtime
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/my-webpage /app/
COPY --from=builder /app/target/site /app/site
WORKDIR /app
EXPOSE 8080
CMD ["/app/my-webpage"]
