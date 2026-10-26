#!/usr/bin/env bash
# set-rt-limits.sh - allow RT scheduling and memlock for user concentrator
set -euo pipefail

LIMITS_FILE="/etc/security/limits.d/concentrator.conf"

cat > "$LIMITS_FILE" <<EOF
concentrator soft rtprio 99
concentrator hard rtprio 99
concentrator soft memlock unlimited
concentrator hard memlock unlimited
concentrator soft core unlimited
concentrator hard core unlimited
EOF

echo "Limits written to $LIMITS_FILE"
echo "Re-login as concentrator to apply"
