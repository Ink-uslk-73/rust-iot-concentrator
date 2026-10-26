//! src/storage/baselines.rs - baseline read/write (DB + files).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::path::{Path, PathBuf};

use super::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baseline {
    pub device_id: String,
    pub version: i32,
    pub hash: String,
    pub captured_at: DateTime<Utc>,
    pub operator: String,
    pub reason: String,
    pub approval_ticket: Option<String>,
    pub payload: serde_json::Value,
    pub git_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineSummary {
    pub device_id: String,
    pub baseline_id: String,
    pub version: i32,
    pub captured_at: DateTime<Utc>,
    pub hash: String,
    pub critical_paths_count: usize,
    pub high_paths_count: usize,
    pub drift_status: String,
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    format!("sha256:{}", hex::encode(h.finalize()))
}

fn file_path(root: &Path, device_id: &str, version: i32) -> PathBuf {
    root.join("baselines").join(device_id).join(format!("v{:04}.json", version))
}

pub async fn save(pool: Option<&PgPool>, root: &Path, b: &Baseline) -> Result<()> {
    let path = file_path(root, &b.device_id, b.version);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(b)?)?;

    if let Some(pool) = pool {
        let res = sqlx::query(
            r#"INSERT INTO baselines
               (device_id, version, hash, captured_at, operator, reason,
                approval_ticket, payload, git_commit)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(&b.device_id).bind(b.version).bind(&b.hash)
        .bind(b.captured_at).bind(&b.operator).bind(&b.reason)
        .bind(&b.approval_ticket).bind(&b.payload).bind(&b.git_commit)
        .execute(pool).await;
        if let Err(e) = res {
            tracing::warn!(error = %e, "baseline DB write failed; file is authoritative");
        }
    }
    Ok(())
}

pub async fn latest(pool: Option<&PgPool>, root: &Path, device_id: &str) -> Result<Option<Baseline>> {
    if let Some(pool) = pool {
        let row = sqlx::query_as::<_, (String, i32, String, DateTime<Utc>, String, String, Option<String>, serde_json::Value, Option<String>)>(
            r#"SELECT device_id, version, hash, captured_at, operator, reason,
                      approval_ticket, payload, git_commit
               FROM baselines WHERE device_id = $1 ORDER BY version DESC LIMIT 1"#
        )
        .bind(device_id)
        .fetch_optional(pool)
        .await;
        match row {
            Ok(Some(r)) => return Ok(Some(Baseline {
                device_id: r.0, version: r.1, hash: r.2, captured_at: r.3,
                operator: r.4, reason: r.5, approval_ticket: r.6,
                payload: r.7, git_commit: r.8,
            })),
            Ok(None) => return Ok(None),
            Err(e) => tracing::warn!(error = %e, "DB read failed; falling back to files"),
        }
    }

    let dir = root.join("baselines").join(device_id);
    if !dir.exists() { return Ok(None); }
    let mut best: Option<(i32, PathBuf)> = None;
    for e in std::fs::read_dir(&dir)? {
        let e = e?;
        let name = e.file_name().to_string_lossy().to_string();
        if let Some(stripped) = name.strip_prefix("v") {
            if let Some(num) = stripped.strip_suffix(".json") {
                if let Ok(v) = num.parse::<i32>() {
                    if best.as_ref().map_or(true, |(bv, _)| v > *bv) {
                        best = Some((v, e.path()));
                    }
                }
            }
        }
    }
    if let Some((_, path)) = best {
        let b: Baseline = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        return Ok(Some(b));
    }
    Ok(None)
}

pub async fn history(pool: Option<&PgPool>, root: &Path, device_id: &str, limit: i64) -> Result<Vec<Baseline>> {
    if let Some(pool) = pool {
        let rows = sqlx::query_as::<_, (String, i32, String, DateTime<Utc>, String, String, Option<String>, serde_json::Value, Option<String>)>(
            r#"SELECT device_id, version, hash, captured_at, operator, reason,
                      approval_ticket, payload, git_commit
               FROM baselines WHERE device_id = $1
               ORDER BY version DESC LIMIT $2"#
        )
        .bind(device_id).bind(limit)
        .fetch_all(pool).await;
        if let Ok(rows) = rows {
            return Ok(rows.into_iter().map(|r| Baseline {
                device_id: r.0, version: r.1, hash: r.2, captured_at: r.3,
                operator: r.4, reason: r.5, approval_ticket: r.6,
                payload: r.7, git_commit: r.8,
            }).collect());
        }
    }
    let dir = root.join("baselines").join(device_id);
    if !dir.exists() { return Ok(Vec::new()); }
    let mut out = Vec::new();
    for e in std::fs::read_dir(&dir)? {
        let path = e?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(c) = std::fs::read_to_string(&path) {
                if let Ok(b) = serde_json::from_str::<Baseline>(&c) {
                    out.push(b);
                }
            }
        }
    }
    out.sort_by_key(|b| std::cmp::Reverse(b.version));
    out.truncate(limit as usize);
    Ok(out)
}

pub fn next_version(existing: &[Baseline]) -> i32 {
    existing.iter().map(|b| b.version).max().unwrap_or(0) + 1
}

pub fn make_id(device_id: &str, version: i32) -> String {
    format!("{}/v{:04}", device_id, version)
}
