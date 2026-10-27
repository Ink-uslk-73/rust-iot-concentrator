#!/usr/bin/env bash
# measure-overhead.sh - measure concentrator overhead.
set -euo pipefail

TABLET_IP="${1:-192.168.30.10}"

echo "=== Baseline: direct (no concentrator) ==="
iperf3 -c "$TABLET_IP" -t 10 -J | jq ".end.sum_received.bits_per_second"

echo "=== Through hub: SSE stream ==="
curl -s "http://127.0.0.1:18443/api/v1/events/stream" --max-time 10 -o /dev/null -w "%{speed_download}\\n"

echo "=== Overhead metrics ==="
curl -s "http://127.0.0.1:18443/api/v1/metrics/overhead" | jq .
