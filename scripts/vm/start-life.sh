#!/usr/bin/env bash
set -euo pipefail
if ! virsh list --all --name | grep -q "^concentrator-life$"; then
  virsh define config/vm/life.xml
fi
virsh start concentrator-life
virsh vcpupin concentrator-life
virsh numatune concentrator-life
numastat -c qemu-kvm
