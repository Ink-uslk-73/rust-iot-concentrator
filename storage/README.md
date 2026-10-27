# scripts/storage/

Ops-скрипты для storage-слоя (PostgreSQL + TimescaleDB).

| Скрипт | Назначение |
|--------|-----------|
| `install-timescaledb.sh` | Установка TimescaleDB на Debian 11 (AIG-502) |
| `check-timescaledb.sh` | Проверка hypertable, aggregates, retention, compression |
| `migrate.sh` | Применение `migrations/*.sql` (включая `002_timescale.sql`) |

## Порядок

```bash
sudo ./install-timescaledb.sh
sudo ./migrate.sh
./check-timescaledb.sh


---

## 4. Права и `.gitignore`

### 4.1. Права на исполнение

```powershell
# После создания
git update-index --chmod=+x scripts/storage/install-timescaledb.sh
git update-index --chmod=+x scripts/storage/check-timescaledb.sh
git update-index --chmod=+x scripts/storage/migrate.sh