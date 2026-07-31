# my-webpage

Repo with code of my Leptos+Axum personal webpage.

Build report page:
https://piotrek1453.github.io/my-webpage/

# Deployment

This page was only tested on Linux with glibc and under FreeBSD's Linuxulator: for hosting on different OS's or Unix-likes with different stdlib you need to build from source (described below)

## From binary

You can either download the latest binary by hand and run it or use a script: scripts/download_and_run_latest_release.sh

## Building from source

Since this project uses sqlx macros that do compile-time type checking against database you need to have a valid instance of Postgres up and set up to work in this app.
For this make sure to set DATABASE_URL environment variable (either in .env file or from commandline) correctly - otherwise sqlx won't be able to type-check against the database and build will fail

After that running **cargo leptos build|watch|serve <optional flags>** should work

If you don't want to host using cargo leptos and prefer to call the binary directly you need to set up the build artifacts in the following structure:

```
.
├── my-webpage
└── site
    ├── favicon.ico
    └── pkg
        ├── my-webpage.css
        ├── my-webpage.js
        └── my-webpage.wasm
```

# Quick dev setup

If you use the development container built from the Dockerfile, PostgreSQL starts automatically inside the container and `DATABASE_URL` defaults to `postgres://postgres@127.0.0.1:5432/mywebpage`.
The manual steps below are for running the app outside that container.

make sure you have Postgres installed locally for example like this on Arch

```
sudo pacman -S postgresql
systemctl enable --now postgresql.service
```

next login to postgres user, initialize DB and set password

```
sudo -i -u postgres
psql
initDB -D /var/lib/postgres/data
ALTER USER postgres PASSWORD '<PASSWORD>';
```

make sure DB is reachable from container: set listen_addresses to '*' (easiest) in /var/lib/postgres/data/postgresql.conf and in pg_hba.conf append line like:

```
host    all    all    10.140.131.0/24    scram-sha-256
```

exact ip and mask can be found by trying to run in container 

```
psql -h host.containers.internal -U postgres -d <DB_NAME>
```

error will show the exact ip

with that you can also find podman net address and mask with 

```
ip route
```

on localhost


next make sure you have .env file in root of repo, example is in example.env. without it container build will fail

If you are using the development container described by the `Dockerfile`, you do NOT need to edit the database settings — simply copy `example.env` to `.env` and the container will start a local PostgreSQL instance automatically. The dev container sets `DATABASE_URL` to `postgres://postgres@127.0.0.1:5432/mywebpage` by default.

Quick commands to prepare and run the dev container locally:

```bash
cp example.env .env
docker build -t my-webpage-dev -f Dockerfile .
docker run --rm -p 8080:8080 -v "$PWD":/app my-webpage-dev
```

If you are running the app outside the dev container, set `DATABASE_URL` in `.env` to point to your Postgres instance, for example:

```
DATABASE_URL=postgres://postgres:<PASSWORD>@host.containers.internal:5432/<DB_NAME>
```

then run migration with

```
sqlx migrate run
```

after that you can build and run for example with

```
cargo leptos watch
```

and the app should connect to DB in Postgres instance on localhost.

## Useful resources
- [Leptos book](https://book.leptos.dev/),
- [start-axum project template](https://github.com/leptos-rs/start-axum),
- [Setting up tailwind4 and daisyUI5 in Leptos 0.8](https://8vi.cat/leptos-0-8-tailwind4-daisyui5-for-easy-websites/)
