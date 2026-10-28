#!/usr/bin/env bash
# check-vm.sh - check VM pinning, numatune, numa_foreign
set -euo pipefail

for vm in concentrator-soft concentrator-life; do
  echo "=== $vm ==="
  virsh vcpupin "$vm" 2>/dev/null || echo "  not defined"
  virsh numatune "$vm" 2>/dev/null || true
done

echo "=== numastat (numa_foreign must be 0) ==="
numastat -c qemu-kvm
