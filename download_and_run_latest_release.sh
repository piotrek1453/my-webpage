#!/bin/sh
set -e

REPO="piotrek1453/my-webpage"
OUT="release.tar.gz"

mkdir -p my-webapp
cd my-webapp
echo "📁 Working directory: ${PWD}"

# Pobierz URL do pierwszego pliku .tar.gz w assets najnowszego release’a
URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | jq -r '.assets[] | select(.name | endswith(".tar.gz")) | .browser_download_url' | head -n 1)

if [ -z "$URL" ]; then
  echo "❌ Nie znaleziono żadnego pliku .tar.gz w release!"
  exit 1
fi

echo "⬇️  Downloading $URL"
curl -L "$URL" -o "$OUT"

# Rozpakuj zawartość do bieżącego katalogu (bez tworzenia podkatalogu)
tar -xzf "$OUT"
rm "$OUT"

echo "🚀 Starting the server binary..."
./my-webpage
