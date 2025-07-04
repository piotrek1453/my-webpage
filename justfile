# justfile
default:
    just watch

init:
    pnpm init
    pnpm install -D tailwindcss daisyui@latest

watch build-type="":
    just watch-tailwind &
    just watch-leptos {{build-type}}

watch-tailwind:
    npx @tailwindcss/cli -i style/tailwind.css -o static/style.css --watch --minify

watch-leptos build-type="":
    cargo leptos watch {{build-type}}
