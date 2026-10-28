#!/usr/bin/env bash
# start-hard.sh - define + start HARD active (standby: not started)
set -euo pipefail

if ! virsh list --all --name | grep -q "^concentrator-hard-active$"; then
  virsh define config/vm/hard-active.xml
fi
if ! virsh list --all --name | grep -q "^concentrator-hard-standby$"; then
  virsh define config/vm/hard-standby.xml
fi

# Active only (standby: not started)
virsh start concentrator-hard-active
virsh vcpupin concentrator-hard-active
virsh numatune concentrator-hard-active

echo "HARD active started. Standby defined but NOT started (overflow mode)."
