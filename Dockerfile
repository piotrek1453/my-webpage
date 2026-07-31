ARG RUST_IMAGE=docker.io/rustlang/rust:nightly-alpine-2026-07-19
ARG MUSLRUST_IMAGE=docker.io/clux/muslrust:1.99.0-nightly
ARG ALPINE_IMAGE=alpine:3.24.1

FROM ${RUST_IMAGE} AS toolchain

RUN apk add --no-cache musl-dev git curl bash

# Install cargo-binstall (prebuilt binary)
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Use binstall to get prebuilt cargo-leptos
RUN cargo binstall cargo-leptos --locked --no-confirm

# Stage 0: Base image with dependencies (for dev/build stages)
FROM ${RUST_IMAGE} AS base
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    npm \
    bash \
    shadow \
    sudo \
    git \
    fish \
    perl \
    make \
    openssh \
    sccache \
    clang \
    llvm \
    mold \
    postgresql \
    binutils && \
    npm i -g pnpm && \
    adduser -D vscode -s /usr/bin/fish && \
    echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
    rustup target add wasm32-unknown-unknown && \
    mkdir -p /app && chown vscode:vscode /app
ENV RUSTC_WRAPPER=sccache
WORKDIR /app
USER vscode
COPY --from=toolchain /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/

FROM ${MUSLRUST_IMAGE} AS planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM ${MUSLRUST_IMAGE} AS cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
RUN cargo chef cook --release --recipe-path recipe.json

FROM base AS builder
COPY --chown=vscode:vscode . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo leptos build --release

FROM base AS dev
RUN RUSTFLAGS="" cargo install leptosfmt sqlx-cli
COPY --chown=vscode:vscode . .
RUN chmod +x scripts/dev-entrypoint.sh
ENV RUST_LOG=info \
    PGDATA=/tmp/my-webpage-postgres \
    DATABASE_URL=postgres://postgres@127.0.0.1:5432/mywebpage
EXPOSE 8080
ENTRYPOINT ["scripts/dev-entrypoint.sh"]
CMD ["cargo", "leptos", "watch", "--hot-reload"]

FROM ${ALPINE_IMAGE} AS runtime
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/my-webpage /app/
COPY --from=builder /app/target/site /app/site
WORKDIR /app
EXPOSE 8080
CMD ["/app/my-webpage"]
