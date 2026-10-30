//! src/app/api.rs - AppState + router + axum::serve.

use std::sync::Arc;
use std::sync::RwLock;

use tokio::net::TcpListener;
use tracing::{info, warn};

use crate::api::{router, AppState};
use crate::config::ConcentratorConfig;
use crate::core::bus::Bus;
use crate::core::mode_switch::ModeState;
use crate::metrics::OverheadMetrics;
use crate::metrics::registry::Registry;
use crate::satellite::SatelliteRegistry;
use crate::storage::cache::HotCache;
use crate::storage::pg;

#[allow(clippy::too_many_arguments)]
pub fn build_state(
    pool: Option<pg::PgPool>,
    metrics: Arc<OverheadMetrics>,
    registry: Arc<Registry>,
    cache: Option<Arc<HotCache>>,
    mode: ModeState,
    bus: Bus,
    config: Arc<RwLock<ConcentratorConfig>>,
    satellites: Arc<SatelliteRegistry>,
) -> Arc<AppState> {
    Arc::new(AppState {
        pool,
        root: std::path::PathBuf::from("."),
        metrics,
        registry,
        cache,
        mode,
        bus,
        config,
        satellites,
    })
}

pub async fn serve(addr: &str, state: Arc<AppState>) -> anyhow::Result<tokio::task::JoinHandle<()>> {
    let app = router(state);
    let listener = TcpListener::bind(addr).await?;
    info!(addr = %addr, "api server listening");
    Ok(tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            warn!("api server error: {}", e);
        }
    }))
}

