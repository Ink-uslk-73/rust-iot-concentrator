//! src/app/storage.rs - PostgreSQL + Redis cache + RawStore + cache_tasks.

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use tracing::warn;

use crate::cache_tasks::spawn_cache_tasks;
use crate::config::ConcentratorConfig;
use crate::core::bus::Bus;
use crate::storage::cache::HotCache;
use crate::storage::pg;
use crate::storage::raw::RawStore;

pub struct StorageHandles {
    pub cache: Option<Arc<HotCache>>,
    pub raw: Arc<RawStore>,
}

pub async fn init(
    database_url: &str,
    cfg: Arc<RwLock<ConcentratorConfig>>,
) -> StorageHandles {
    let _pool = match pg::connect(database_url).await {
        Ok(p) => { tracing::info!("postgresql connected"); Some(p) }
        Err(e) => { warn!(error = %e, "postgresql unavailable; file-fallback mode"); None }
    };

    let cache: Option<Arc<HotCache>> = {
        let c = cfg.read().unwrap();
        if c.cache.enabled {
            match (c.cache.url.as_deref(), c.cache.ttl_sec) {
                (Some(url), Some(ttl)) => match HotCache::new(url, ttl) {
                    Ok(hc) => { tracing::info!(url, ttl, "redis cache enabled"); Some(Arc::new(hc)) }
                    Err(e) => { warn!("redis init: {}", e); None }
                },
                _ => { warn!("cache.enabled=true but url/ttl missing"); None }
            }
        } else { None }
    };

    let raw_path = cfg.read().unwrap().cache.raw_path.clone()
        .unwrap_or_else(|| "storage/raw/telemetry.cbor".into());
    let raw = Arc::new(RawStore::new(&raw_path));

    StorageHandles { cache, raw }
}

pub fn spawn_tasks(
    cache: Option<Arc<HotCache>>,
    raw: Arc<RawStore>,
    pool: Option<crate::storage::pg::PgPool>,
    bus: Bus,
    cfg: Arc<RwLock<ConcentratorConfig>>,
) {
    let flush_sec = cfg.read().unwrap().cache.flush_interval_sec.unwrap_or(60);
    spawn_cache_tasks(cache, raw, pool, bus, Duration::from_secs(flush_sec), Duration::from_secs(60));
}

