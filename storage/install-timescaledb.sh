#!/usr/bin/env bash
# install-timescaledb.sh - установка TimescaleDB на Debian 11 (AIG-502)
set -euo pipefail

if [[ $EUID -ne 0 ]]; then
  echo "Run as root (sudo)" >&2
  exit 1
fi

echo "deb https://packagecloud.io/timescale/timescaledb/debian/ bullseye main" \
  > /etc/apt/sources.list.d/timescaledb.list
wget --quiet -O - https://packagecloud.io/timescale/timescaledb/gpgkey | apt-key add -

apt-get update
apt-get install -y timescaledb-2-postgresql-14

timescaledb-tune --quiet --yes
systemctl restart postgresql

sudo -u postgres psql -c "CREATE EXTENSION IF NOT EXISTS timescaledb;"
echo "TimescaleDB installed."