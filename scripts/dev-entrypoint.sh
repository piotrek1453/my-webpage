#!/usr/bin/env sh
set -eu

PGDATA_DIR="${PGDATA:-/tmp/my-webpage-postgres}"
PGHOST_ADDR="127.0.0.1"
PGPORT_NUM="5432"
APP_DB_NAME="mywebpage"
BOOTSTRAP_DB_USER="$(id -un)"
DEFAULT_DATABASE_URL="postgres://postgres@${PGHOST_ADDR}:${PGPORT_NUM}/${APP_DB_NAME}"

if [ -z "${DATABASE_URL:-}" ]; then
    export DATABASE_URL="$DEFAULT_DATABASE_URL"
fi

if [ ! -d "$PGDATA_DIR" ] || [ ! -f "$PGDATA_DIR/PG_VERSION" ]; then
    mkdir -p "$PGDATA_DIR"
    initdb -D "$PGDATA_DIR" --auth=trust --username="$BOOTSTRAP_DB_USER" >/dev/null
fi

if ! pg_isready -h "$PGHOST_ADDR" -p "$PGPORT_NUM" >/dev/null 2>&1; then
    pg_ctl -D "$PGDATA_DIR" -o "-c listen_addresses=${PGHOST_ADDR} -p ${PGPORT_NUM} -k /tmp" -w start >/dev/null
fi

cleanup() {
    if [ -n "${APP_PID:-}" ]; then
        kill "$APP_PID" >/dev/null 2>&1 || true
    fi
    pg_ctl -D "$PGDATA_DIR" stop -m fast >/dev/null 2>&1 || true
}

trap cleanup INT TERM EXIT

until pg_isready -h "$PGHOST_ADDR" -p "$PGPORT_NUM" >/dev/null 2>&1; do
    sleep 1
done

createdb -h "$PGHOST_ADDR" -p "$PGPORT_NUM" postgres >/dev/null 2>&1 || true

if [ "$(psql -h "$PGHOST_ADDR" -p "$PGPORT_NUM" -U "$BOOTSTRAP_DB_USER" -d postgres -tAc "SELECT 1 FROM pg_roles WHERE rolname = 'postgres'" | tr -d '[:space:]')" != "1" ]; then
    psql -h "$PGHOST_ADDR" -p "$PGPORT_NUM" -U "$BOOTSTRAP_DB_USER" -d postgres -c "CREATE ROLE postgres WITH LOGIN SUPERUSER" >/dev/null
fi

createdb -h "$PGHOST_ADDR" -p "$PGPORT_NUM" -U "$BOOTSTRAP_DB_USER" "$APP_DB_NAME" >/dev/null 2>&1 || true

"$@" &
APP_PID="$!"
wait "$APP_PID"