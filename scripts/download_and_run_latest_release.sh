#!/bin/sh
set -e
PATH="/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin"
export PATH

REPO="piotrek1453/my-webpage"

# Detect platform and set workdir
case "$(uname -s)" in
  Linux*)   PLATFORM="linux-x86_64"; WORKDIR="/srv/www/my-webpage" ;;
  FreeBSD*) PLATFORM="freebsd-x86_64"; WORKDIR="/usr/local/www/my-webpage" ;;
  *)        PLATFORM="linux-x86_64"; WORKDIR="/srv/www/my-webpage" ;;
esac

TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | jq -r .tag_name)
OUT="release.tar.gz"
ARCHIVE="my-webpage-${TAG}-${PLATFORM}.tar.gz"

mkdir -p $WORKDIR
rm -rf "${WORKDIR:?}/"*
cd $WORKDIR
echo "📁 Working directory: ${PWD}"

# Download latest release archive for detected platform
URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | jq -r --arg ARCHIVE "$ARCHIVE" '.assets[] | select(.name == $ARCHIVE) | .browser_download_url')

if [ -z "$URL" ]; then
  echo "❌ No file found for $ARCHIVE in release!"
  exit 1
fi

echo "⬇  Downloading $URL"
curl -L "$URL" -o "$OUT"

# Unpack archive
tar -xzf "$OUT"
rm "$OUT"

# Place correct dotenv
echo "$PWD"
cp -f ~/.env .env

# Run server
echo "🚀 Starting the server binary..."
if command -v daemon >/dev/null 2>&1; then
  /usr/sbin/daemon -f -o "$WORKDIR/my-webpage.log" "$WORKDIR/my-webpage"
else
  nohup "$WORKDIR/my-webpage" >> "$WORKDIR/my-webpage.log" 2>&1 &
fi
