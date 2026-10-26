//! src/storage/fallback.rs - sync JSON files -> PostgreSQL after DB recovery.

use sqlx::PgPool;
use std::path::Path;

use super::Result;

pub async fn sync_baselines(pool: &PgPool, root: &Path) -> Result<usize> {
    let dir = root.join("baselines");
    if !dir.exists() { return Ok(0); }
    let mut count = 0usize;
    for dev in std::fs::read_dir(&dir)? {
        let dev_path = dev?.path();
        if !dev_path.is_dir() { continue; }
        for f in std::fs::read_dir(&dev_path)? {
            let p = f?.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json") { continue; }
            let Ok(c) = std::fs::read_to_string(&p) else { continue };
            let Ok(b) = serde_json::from_str::<super::baselines::Baseline>(&c) else { continue };
            let res = sqlx::query(
                r#"INSERT INTO baselines
                   (device_id, version, hash, captured_at, operator, reason,
                    approval_ticket, payload, git_commit)
                   VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
                   ON CONFLICT (device_id, version) DO NOTHING"#
            )
            .bind(&b.device_id).bind(b.version).bind(&b.hash)
            .bind(b.captured_at).bind(&b.operator).bind(&b.reason)
            .bind(&b.approval_ticket).bind(&b.payload).bind(&b.git_commit)
            .execute(pool).await;
            if res.is_ok() { count += 1; }
        }
    }
    Ok(count)
}

pub async fn sync_events(pool: &PgPool, root: &Path) -> Result<usize> {
    let dir = root.join("events");
    if !dir.exists() { return Ok(0); }
    let mut count = 0usize;
    for dev in std::fs::read_dir(&dir)? {
        let dev_path = dev?.path();
        if !dev_path.is_dir() { continue; }
        for day in std::fs::read_dir(&dev_path)? {
            let day_path = day?.path();
            if !day_path.is_dir() { continue; }
            for f in std::fs::read_dir(&day_path)? {
                let p = f?.path();
                if p.extension().and_then(|s| s.to_str()) != Some("json") { continue; }
                let Ok(c) = std::fs::read_to_string(&p) else { continue };
                let Ok(e) = serde_json::from_str::<super::events::Event>(&c) else { continue };
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
                if res.is_ok() { count += 1; }
            }
        }
    }
    Ok(count)
}
