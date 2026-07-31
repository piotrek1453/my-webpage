#!/usr/bin/env sh
set -eu

pnpm install daisyui@"${DAISYUI_VERSION:-latest}"
sqlx migrate run
exec cargo leptos watch --hot-reload