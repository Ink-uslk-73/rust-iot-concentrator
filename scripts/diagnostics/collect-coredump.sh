#!/usr/bin/env bash
# collect-coredump.sh - gather latest coredump + metadata into /var/crash
set -euo pipefail

OUT_DIR="/var/crash/archive"
mkdir -p "$OUT_DIR"

LATEST=$(coredumpctl list -1 --no-pager --no-legend 2>/dev/null | awk "{print \$5}" | tail -1 || true)

if [ -z "${LATEST:-}" ]; then
    echo "No coredumps found"
    exit 0
fi

TS=$(date +%Y%m%d_%H%M%S)
BASE="$OUT_DIR/concentrator_$TS"

coredumpctl dump "$LATEST" -o "$BASE.core" 2>/dev/null || true
coredumpctl info "$LATEST" > "$BASE.info" 2>/dev/null || true

echo "Saved: $BASE.core + $BASE.info"
