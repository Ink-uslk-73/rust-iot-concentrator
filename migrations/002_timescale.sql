-- 002_timescale.sql - hypertable + continuous aggregates + retention + compression
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- 1. Hypertable for events
SELECT create_hypertable('events', 'ts', if_not_exists => TRUE, migrate_data => TRUE);

-- 2. Continuous aggregates (1m, 5m, 1h, 24h)
CREATE MATERIALIZED VIEW IF NOT EXISTS events_1m
WITH (timescaledb.continuous) AS
SELECT time_bucket('1 minute', ts) AS bucket,
       device_id, severity, technique_id,
       count(*) AS cnt
FROM events
GROUP BY bucket, device_id, severity, technique_id
WITH NO DATA;

-- 3. Retention (90 days)
SELECT add_retention_policy('events', INTERVAL '90 days', if_not_exists => TRUE);

-- 4. Compression (7 days)
ALTER TABLE events SET (
  timescaledb.compress,
  timescaledb.compress_segmentby = 'device_id,severity',
  timescaledb.compress_orderby = 'ts DESC'
);
SELECT add_compression_policy('events', INTERVAL '7 days', if_not_exists => TRUE);

-- 5. Refresh policy (start_offset > drop_after? NO: 2h < 90d - correct)
CALL refresh_continuous_aggregate('events_1m', NULL, NULL);
SELECT add_continuous_aggregate_policy('events_1m',
  start_offset => INTERVAL '2 hours',
  end_offset   => INTERVAL '1 minute',
  schedule_interval => INTERVAL '1 minute',
  if_not_exists => TRUE
);
