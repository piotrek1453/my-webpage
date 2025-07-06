# justfile
default:
    just watch

init:
    pnpm init
    pnpm install -D tailwindcss daisyui@latest
    curl -L https://daisyui.com/llms.txt --create-dirs -o .vscode/daisyui.md

watch build-type="":
    just watch-tailwind &
    just watch-leptos {{build-type}}

watch-tailwind:
    pnpx @tailwindcss/cli -i style/tailwind.css -o static/style.css --watch

watch-leptos build-type="":
    cargo leptos watch {{build-type}}
