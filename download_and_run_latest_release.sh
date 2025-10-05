#!/bin/sh
set -e

REPO="piotrek1453/my-webpage"
OUT="release.tar.gz"

mkdir -p my-webpage
cd my-webpage
echo "📁 Working directory: ${PWD}"

# Download latest release archive
URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | jq -r '.assets[] | select(.name | endswith(".tar.gz")) | .browser_download_url' | head -n 1)

if [ -z "$URL" ]; then
  echo "❌ No file found in .tar.gz in release!"
  exit 1
fi

echo "⬇️  Downloading $URL"
curl -L "$URL" -o "$OUT"

# Unpack archive
tar -xzf "$OUT"
rm "$OUT"

# Place correct dotenv
echo $PWD
cp ~/.env .env

# Run server
echo "🚀 Starting the server binary..."
./my-webpage
