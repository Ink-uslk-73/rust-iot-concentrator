-- 001_initial.sql - baseline + events + audit

CREATE TABLE IF NOT EXISTS baselines (
    id              BIGSERIAL PRIMARY KEY,
    device_id       TEXT NOT NULL,
    version         INT  NOT NULL,
    hash            TEXT NOT NULL,
    captured_at     TIMESTAMPTZ NOT NULL,
    operator        TEXT NOT NULL,
    reason          TEXT NOT NULL,
    approval_ticket TEXT,
    payload         JSONB NOT NULL,
    git_commit      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (device_id, version)
);
CREATE INDEX IF NOT EXISTS idx_baselines_device ON baselines (device_id, version DESC);

CREATE TABLE IF NOT EXISTS events (
    id           BIGSERIAL PRIMARY KEY,
    event_id     TEXT NOT NULL UNIQUE,
    ts           TIMESTAMPTZ NOT NULL,
    device_id    TEXT NOT NULL,
    event_type   TEXT NOT NULL,
    severity     TEXT NOT NULL,
    technique_id TEXT,
    tactic       TEXT,
    details      JSONB NOT NULL,
    source       TEXT NOT NULL,
    acked        BOOLEAN NOT NULL DEFAULT false,
    acked_by     TEXT,
    acked_at     TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_events_ts       ON events (ts DESC);
CREATE INDEX IF NOT EXISTS idx_events_device   ON events (device_id, ts DESC);
CREATE INDEX IF NOT EXISTS idx_events_severity ON events (severity, ts DESC);
CREATE INDEX IF NOT EXISTS idx_events_attck    ON events (technique_id);

CREATE TABLE IF NOT EXISTS audit_log (
    id      BIGSERIAL PRIMARY KEY,
    ts      TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor   TEXT NOT NULL,
    action  TEXT NOT NULL,
    target  TEXT NOT NULL,
    details JSONB NOT NULL
);
