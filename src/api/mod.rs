//! src/api/mod.rs - REST + SSE for tablet and SOC.

pub mod rest;
pub mod rbac;
pub mod chat;
pub use rest::*;

use axum::{routing::{get, post}, Router, middleware};
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::config::ConcentratorConfig;
use crate::core::bus::Bus;
use crate::core::mode_switch::ModeState;
use crate::gateway::ui;
use crate::metrics::registry::Registry;
use crate::metrics::OverheadMetrics;
use crate::satellite::SatelliteRegistry;
use crate::storage::cache::HotCache;

#[derive(Clone)]
pub struct AppState {
    pub pool: Option<PgPool>,
    pub root: PathBuf,
    pub metrics: Arc<OverheadMetrics>,
    pub registry: Arc<Registry>,
    pub cache: Option<Arc<HotCache>>,
    pub mode: ModeState,
    pub bus: Bus,
    pub config: Arc<RwLock<ConcentratorConfig>>,
    pub satellites: Arc<SatelliteRegistry>,
    pub telemetry_tx: Option<tokio::sync::broadcast::Sender<crate::core::bus::BusMessage>>,
    pub chat_tx: Option<tokio::sync::broadcast::Sender<chat::ChatMsg>>,
}

pub type SharedState = Arc<AppState>;

pub fn router(state: SharedState) -> Router {
    async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "frontier: ok",
        "version": env!("CARGO_PKG_VERSION"),
        "name": "frontier",
    }))
}
    let read = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/devices/:device_id/baseline", get(baselines::get_latest))
        .route("/api/v1/devices/:device_id/baseline/summary", get(baselines::get_summary))
        .route("/api/v1/devices/:device_id/baseline/diff", get(baselines::get_diff))
        .route("/api/v1/devices/:device_id/baseline/history", get(baselines::get_history))
        .route("/api/v1/events", get(events::list))
        .route("/api/v1/events/stream", get(events::stream))
        .route("/api/v1/events/:event_id", get(events::get_one))
        .route("/api/v1/metrics/overhead", get(metrics::overhead))
        .route("/api/v1/metrics/self", get(self_metrics::self_metrics))
        .route("/api/v1/mode", get(mode::get_mode))
        .route("/api/v1/config/mode", get(config::get_config_mode))
        .route("/api/v1/telemetry/latest/:sensor_id", get(telemetry::latest))
        .route("/api/v1/satellites", get(satellites::list))
        .route("/api/v1/chat/stream", get(chat::stream_chat));

    let write = Router::new()
        .route("/api/v1/devices/:device_id/baseline/capture", post(baselines::post_capture))
        .route("/api/v1/events/ack", post(events::ack))
        .route("/api/v1/mode", post(mode::set_mode))
        .route("/api/v1/config/mode", post(config::set_config_mode))
        .route("/api/v1/chat", post(chat::post_chat))
        .layer(middleware::from_fn(rbac::require_role));

    read.merge(write).fallback(ui::serve).with_state(state)
}
