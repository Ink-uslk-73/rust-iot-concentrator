#!/usr/bin/env bash
# setup-isolcpus.sh - configure GRUB for isolated cores (HARD + LIFE)
set -euo pipefail

GRUB_FILE="/etc/default/grub"
ISOLATED="0-7,64-71"

if ! grep -q "isolcpus=" "$GRUB_FILE"; then
    sed -i "s|^GRUB_CMDLINE_LINUX=\"|GRUB_CMDLINE_LINUX=\"isolcpus=$ISOLATED nohz_full=$ISOLATED rcu_nocbs=$ISOLATED |" "$GRUB_FILE"
    echo "Added isolcpus to $GRUB_FILE"
else
    echo "isolcpus already present in $GRUB_FILE"
fi

update-grub
echo "Reboot required: sudo reboot"
