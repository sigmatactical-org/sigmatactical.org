#!/usr/bin/env bash
# Populate build/image/ for `docker build -f Dockerfile build/image`.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/release/sigmatactical-org"
if [[ ! -f "$BIN" && -f "$ROOT/../../target/release/sigmatactical-org" ]]; then
  BIN="$ROOT/../../target/release/sigmatactical-org"
elif [[ ! -f "$BIN" && -f "$ROOT/../target/release/sigmatactical-org" ]]; then
  BIN="$ROOT/../target/release/sigmatactical-org"
fi
if [[ ! -f "$BIN" ]]; then
  echo "error: missing $BIN — run: cargo build --release" >&2
  exit 1
fi

if [[ -d "$ROOT/../theme/assets/static" ]]; then
  THEME_STATIC="$ROOT/../theme/assets/static"
elif [[ -d "$ROOT/theme/assets/static" ]]; then
  THEME_STATIC="$ROOT/theme/assets/static"
else
  echo "error: theme static assets not found — run ./scripts/prepare-local.sh" >&2
  exit 1
fi

normalize_tree() {
  local dir="$1"
  find "$dir" -type d -exec chmod 755 {} \;
  find "$dir" -type f -exec chmod 644 {} \;
}

mkdir -p "$ROOT/build/image"
rm -f "$ROOT/build/image/sigmatactical-org"
cp "$BIN" "$ROOT/build/image/sigmatactical-org"
chmod 555 "$ROOT/build/image/sigmatactical-org"
rm -rf "$ROOT/build/image/static"
cp -a "$THEME_STATIC" "$ROOT/build/image/static"
normalize_tree "$ROOT/build/image/static"
