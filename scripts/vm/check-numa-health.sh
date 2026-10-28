#!/usr/bin/env bash
set -euo pipefail
echo "=== numastat (numa_foreign must be 0) ==="
numastat -c qemu-kvm
for vm in concentrator-hard-active concentrator-hard-standby concentrator-soft concentrator-life; do
  virsh numatune "$vm" 2>/dev/null || true
done
