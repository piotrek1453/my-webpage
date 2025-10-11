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

## Useful resources
- [Leptos book](https://book.leptos.dev/),
- [start-axum project template](https://github.com/leptos-rs/start-axum),
- [Setting up tailwind4 and daisyUI5 in Leptos 0.8](https://8vi.cat/leptos-0-8-tailwind4-daisyui5-for-easy-websites/)
