-- 003_telemetry.sql - hypertable for telemetry (Redis -> Raw -> TimescaleDB)
CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS telemetry (
    id        BIGSERIAL,
    ts        TIMESTAMPTZ NOT NULL,
    sensor_id TEXT NOT NULL,
    value     DOUBLE PRECISION NOT NULL,
    unit      TEXT,
    source    TEXT NOT NULL DEFAULT 'raw'
);

DO $$
BEGIN
    PERFORM create_hypertable('telemetry', 'ts', if_not_exists => TRUE, migrate_data => TRUE);
EXCEPTION WHEN undefined_function THEN
    RAISE NOTICE 'TimescaleDB not available - regular table';
END
$$;

CREATE INDEX IF NOT EXISTS idx_telemetry_sensor_ts ON telemetry (sensor_id, ts DESC);
