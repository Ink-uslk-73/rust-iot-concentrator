#!/usr/bin/env bash
# check-numa-health.sh - numastat (numa_foreign must be 0)
set -euo pipefail

echo "=== numastat (numa_foreign must be 0 for HARD) ==="
numastat -c qemu-kvm

echo "=== virsh numatune (all VMs) ==="
for vm in concentrator-hard-active concentrator-hard-standby concentrator-soft concentrator-life; do
  virsh numatune "$vm" 2>/dev/null || true
done

echo "=== If numa_foreign > 0: alert (DiagLevel::Critical) ==="
