//! src/cache_tasks.rs - Фоновые задачи: flush (Redis->Raw) + sync (Raw->TimescaleDB).
//!
//! Диагностика: tracing (info/warn) + Bus.diagnostics (Warning при ошибках).

use std::sync::Arc;
use std::time::Duration;
use sqlx::PgPool;
use tracing::{info, warn};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::storage::cache::HotCache;
use crate::storage::raw::RawStore;

fn diag(bus: &Bus, level: DiagLevel, msg: String) {
    let _ = bus.diagnostics.try_send(BusMessage::new(
        "cache_tasks",
        Priority::Life,
        BusPayload::Diagnostic { level, component: "cache_tasks".into(), message: msg },
    ));
}

/// Запускает фоновые задачи:
/// - flush (если cache Some): Redis -> Raw каждые `flush_interval`;
/// - sync (всегда): Raw -> TimescaleDB каждые 60 с (no-op при pool=None).
pub fn spawn_cache_tasks(
    cache: Option<Arc<HotCache>>,
    raw: Arc<RawStore>,
    pool: Option<PgPool>,
    bus: Bus,
    flush_interval: Duration,
    sync_interval: Duration,
) {
    // 1. flush: Redis -> Raw
    if let Some(cache) = cache {
        let raw = raw.clone();
        let bus = bus.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(flush_interval);
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            info!(sec = flush_interval.as_secs(), "cache flush task started");
            loop {
                tick.tick().await;
                match cache.flush_to_raw(&raw).await {
                    Ok(0) => {},
                    Ok(n) => info!(count = n, "flushed Redis -> Raw"),
                    Err(e) => {
                        warn!("flush_to_raw: {}", e);
                        diag(&bus, DiagLevel::Warning, format!("flush_to_raw: {}", e));
                    }
                }
            }
        });
    }

    // 2. sync: Raw -> TimescaleDB
    {
        let raw = raw.clone();
        let pool = pool.clone();
        let bus = bus.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(sync_interval);
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            info!(sec = sync_interval.as_secs(), "cache sync task started");
            loop {
                tick.tick().await;
                match raw.sync_to_timescale(pool.as_ref()).await {
                    Ok(0) => {},
                    Ok(n) => info!(count = n, "synced Raw -> TimescaleDB"),
                    Err(e) => {
                        warn!("sync_to_timescale: {}", e);
                        diag(&bus, DiagLevel::Warning, format!("sync_to_timescale: {}", e));
                    }
                }
            }
        });
    }
}
