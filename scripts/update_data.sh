#!/usr/bin/env bash
set -euo pipefail

BASE_URL="https://raw.githubusercontent.com/sherlock-project/sherlock/refs/heads/master/sherlock_project/resources"
DEST_DIR="$(dirname "$0")/../crates/sherlock/src/resources"

echo "[*] Downloading data.json..."
curl -fSL "$BASE_URL/data.json" -o "$DEST_DIR/data.json"

echo "[*] Downloading data.schema.json..."
curl -fSL "$BASE_URL/data.schema.json" -o "$DEST_DIR/data.schema.json"

echo "[*] Updated $(wc -l < "$DEST_DIR/data.json" | tr -d ' ') lines in $DEST_DIR/data.json"
echo "[*] Done"
