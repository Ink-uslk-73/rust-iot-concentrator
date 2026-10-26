#!/usr/bin/env bash
# numa-report.sh - report NUMA topology and per-node memory
set -euo pipefail

echo "=== numactl --hardware ==="
numactl --hardware || true

echo
echo "=== /sys/devices/system/node ==="
for n in /sys/devices/system/node/node*; do
    echo "$n: $(cat $n/cpulist 2>/dev/null || echo n/a)"
done

echo
echo "=== per-node meminfo ==="
for n in /sys/devices/system/node/node*; do
    echo "--- $n ---"
    grep -E "MemTotal|MemFree" "$n/meminfo" 2>/dev/null || true
done
