#!/usr/bin/env bash
# install-timescaledb.sh - install TimescaleDB on Debian 11 (AIG-502)
set -euo pipefail

if [[ $EUID -ne 0 ]]; then
  echo "Run as root (sudo)" >&2
  exit 1
fi

# 1. PostgreSQL third-party repository
apt install -y gnupg postgresql-common apt-transport-https lsb-release wget
/usr/share/postgresql-common/pgdg/apt.postgresql.org.sh -y

# 2. TimescaleDB repository
echo "deb https://packagecloud.io/timescale/timescaledb/debian/ $(lsb_release -c -s) main" \
  > /etc/apt/sources.list.d/timescaledb.list
wget --quiet -O - https://packagecloud.io/timescale/timescaledb/gpgkey | apt-key add -

# 3. Install TimescaleDB (PostgreSQL 14)
apt update
apt install -y timescaledb-2-postgresql-14

# 4. Tune + restart
timescaledb-tune --quiet --yes
systemctl restart postgresql

# 5. Verify
sudo -u postgres psql -c "CREATE EXTENSION IF NOT EXISTS timescaledb;"
sudo -u postgres psql -c "SELECT extversion FROM pg_extension WHERE extname = 'timescaledb';"
echo "TimescaleDB installed."
