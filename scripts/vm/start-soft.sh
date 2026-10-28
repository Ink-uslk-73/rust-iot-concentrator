#!/usr/bin/env bash
set -euo pipefail
if ! virsh list --all --name | grep -q "^concentrator-soft$"; then
  virsh define config/vm/soft.xml
fi
virsh start concentrator-soft
virsh vcpupin concentrator-soft
virsh numatune concentrator-soft
numastat -c qemu-kvm
