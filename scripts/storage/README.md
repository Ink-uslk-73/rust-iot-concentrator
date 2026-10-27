# scripts/storage/

Ops scripts for storage layer (PostgreSQL + TimescaleDB).

| Script | Purpose |
|--------|---------|
| `install-timescaledb.sh` | Install TimescaleDB on Debian 11 (AIG-502) |
| `check-timescaledb.sh` | Verify hypertable, aggregates, retention, compression |
| `migrate.sh` | Apply `migrations/*.sql` |

## Order

```bash
sudo ./install-timescaledb.sh
sudo ./migrate.sh
./check-timescaledb.sh
```

## Important

- `start_offset` continuous aggregate **>** `drop_after` retention? **NO** for our case:
  `start_offset = 2h`, `drop_after = 90d` -> `2h < 90d` (correct).
- `baselines` **not** compressed (JSONB, UPDATE).
- Multi-node TimescaleDB - **not now** (single server, standby enough).
