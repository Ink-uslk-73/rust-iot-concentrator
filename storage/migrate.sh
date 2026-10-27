#!/usr/bin/env bash
# migrate.sh - применение migrations/*.sql (включая 002_timescale.sql)
set -euo pipefail

DB="${DB:-concentrator}"
MIG_DIR="$(dirname "$0")/../../migrations"

for f in "$MIG_DIR"/*.sql; do
  echo ">>> applying $f"
  sudo -u postgres psql -d "$DB" -f "$f"
done

echo "Migrations applied."