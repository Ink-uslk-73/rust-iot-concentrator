//! src/storage/events.rs - events read/write (DB + files).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::path::Path;

use super::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub ts: DateTime<Utc>,
    pub device_id: String,
    pub event_type: String,
    pub severity: String,
    pub technique_id: Option<String>,
    pub tactic: Option<String>,
    pub details: serde_json::Value,
    pub source: String,
    pub acked: bool,
    pub acked_by: Option<String>,
    pub acked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EventFilter {
    pub device_id: Option<String>,
    pub severity: Option<String>,
    pub technique: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
}

fn event_file(root: &Path, e: &Event) -> std::path::PathBuf {
    let day = e.ts.format("%Y-%m-%d").to_string();
    root.join("events").join(&e.device_id).join(day)
        .join(format!("{}.json", e.event_id))
}

pub async fn append(pool: Option<&PgPool>, root: &Path, e: &Event) -> Result<()> {
    let path = event_file(root, e);
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    std::fs::write(&path, serde_json::to_string_pretty(e)?)?;

    if let Some(pool) = pool {
        let res = sqlx::query(
            r#"INSERT INTO events
               (event_id, ts, device_id, event_type, severity, technique_id, tactic,
                details, source, acked, acked_by, acked_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
               ON CONFLICT (event_id) DO NOTHING"#
        )
        .bind(&e.event_id).bind(e.ts).bind(&e.device_id)
        .bind(&e.event_type).bind(&e.severity).bind(&e.technique_id)
        .bind(&e.tactic).bind(&e.details).bind(&e.source)
        .bind(e.acked).bind(&e.acked_by).bind(e.acked_at)
        .execute(pool).await;
        if let Err(err) = res {
            tracing::warn!(error = %err, "event DB write failed; file is authoritative");
        }
    }
    Ok(())
}

pub async fn list(pool: Option<&PgPool>, f: &EventFilter) -> Result<Vec<Event>> {
    let limit = f.limit.unwrap_or(100).min(1000);
    if let Some(pool) = pool {
        let rows = sqlx::query_as::<_, (String, DateTime<Utc>, String, String, String, Option<String>, Option<String>, serde_json::Value, String, bool, Option<String>, Option<DateTime<Utc>>)>(
            r#"SELECT event_id, ts, device_id, event_type, severity, technique_id, tactic,
                      details, source, acked, acked_by, acked_at
               FROM events
               WHERE ($1::text IS NULL OR device_id   = $1)
                 AND ($2::text IS NULL OR severity     = $2)
                 AND ($3::text IS NULL OR technique_id = $3)
                 AND ($4::timestamptz IS NULL OR ts    >= $4)
               ORDER BY ts DESC LIMIT $5"#
        )
        .bind(f.device_id.as_deref()).bind(f.severity.as_deref())
        .bind(f.technique.as_deref()).bind(f.since).bind(limit)
        .fetch_all(pool).await;
        if let Ok(rows) = rows {
            return Ok(rows.into_iter().map(|r| Event {
                event_id: r.0, ts: r.1, device_id: r.2, event_type: r.3,
                severity: r.4, technique_id: r.5, tactic: r.6, details: r.7,
                source: r.8, acked: r.9, acked_by: r.10, acked_at: r.11,
            }).collect());
        }
    }
    Ok(Vec::new())
}

pub async fn get_one(pool: Option<&PgPool>, event_id: &str) -> Result<Option<Event>> {
    if let Some(pool) = pool {
        let row = sqlx::query_as::<_, (String, DateTime<Utc>, String, String, String, Option<String>, Option<String>, serde_json::Value, String, bool, Option<String>, Option<DateTime<Utc>>)>(
            r#"SELECT event_id, ts, device_id, event_type, severity, technique_id, tactic,
                      details, source, acked, acked_by, acked_at
               FROM events WHERE event_id = $1"#
        )
        .bind(event_id)
        .fetch_optional(pool).await;
        if let Ok(Some(r)) = row {
            return Ok(Some(Event {
                event_id: r.0, ts: r.1, device_id: r.2, event_type: r.3,
                severity: r.4, technique_id: r.5, tactic: r.6, details: r.7,
                source: r.8, acked: r.9, acked_by: r.10, acked_at: r.11,
            }));
        }
    }
    Ok(None)
}

pub async fn ack(pool: Option<&PgPool>, event_id: &str, by: &str) -> Result<()> {
    if let Some(pool) = pool {
        sqlx::query(
            "UPDATE events SET acked = true, acked_by = $1, acked_at = now() WHERE event_id = $2"
        )
        .bind(by).bind(event_id)
        .execute(pool).await?;
    }
    Ok(())
}
