//! src/storage/telemetry.rs - telemetry read (TimescaleDB).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TelemetryRow {
    pub ts: DateTime<Utc>,
    pub sensor_id: String,
    pub value: f64,
    pub unit: Option<String>,
    pub source: String,
}

pub async fn latest(pool: Option<&PgPool>, sensor_id: &str) -> Result<Option<TelemetryRow>> {
    let Some(pool) = pool else { return Ok(None); };
    let row = sqlx::query_as::<_, TelemetryRow>(
        r#"SELECT ts, sensor_id, value, unit, source
           FROM telemetry WHERE sensor_id = $1
           ORDER BY ts DESC LIMIT 1"#
    )
    .bind(sensor_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn history(pool: Option<&PgPool>, sensor_id: &str, limit: i64) -> Result<Vec<TelemetryRow>> {
    let Some(pool) = pool else { return Ok(Vec::new()); };
    let rows = sqlx::query_as::<_, TelemetryRow>(
        r#"SELECT ts, sensor_id, value, unit, source
           FROM telemetry WHERE sensor_id = $1
           ORDER BY ts DESC LIMIT $2"#
    )
    .bind(sensor_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
