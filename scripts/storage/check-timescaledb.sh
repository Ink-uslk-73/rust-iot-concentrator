#!/usr/bin/env bash
# check-timescaledb.sh - verify hypertable, aggregates, retention, compression
set -euo pipefail

PSQL="sudo -u postgres psql -d concentrator -t -A"

echo "=== version ==="
$PSQL -c "SELECT extversion FROM pg_extension WHERE extname = 'timescaledb';"

echo "=== hypertables ==="
$PSQL -c "SELECT hypertable_name FROM timescaledb_information.hypertables;"

echo "=== continuous aggregates ==="
$PSQL -c "SELECT view_name FROM timescaledb_information.continuous_aggregates;"

echo "=== retention ==="
$PSQL -c "SELECT hypertable_name, drop_after FROM timescaledb_information.jobs WHERE proc_name = 'policy_retention';"

echo "=== compression ==="
$PSQL -c "SELECT hypertable_name, compress_after FROM timescaledb_information.jobs WHERE proc_name = 'policy_compression';"

echo "=== compression stats ==="
$PSQL -c "SELECT chunk_name, before_compression_total_bytes, after_compression_total_bytes FROM chunk_compression_stats('events');"
