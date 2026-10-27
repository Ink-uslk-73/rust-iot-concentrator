//! src/api/mod.rs - REST + SSE for tablet and SOC.

pub mod baselines;
pub mod events;
pub mod metrics;
pub mod mode;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::bus::Bus;
use crate::core::mode_switch::ModeState;
use crate::metrics::OverheadMetrics;

#[derive(Clone)]
pub struct AppState {
    pub pool: Option<PgPool>,
    pub root: PathBuf,
    pub metrics: Arc<OverheadMetrics>,
    pub mode: ModeState,
    pub bus: Bus,
}

pub type SharedState = Arc<AppState>;

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/devices/:device_id/baseline",          get(baselines::get_latest))
        .route("/api/v1/devices/:device_id/baseline/summary",  get(baselines::get_summary))
        .route("/api/v1/devices/:device_id/baseline/diff",     get(baselines::get_diff))
        .route("/api/v1/devices/:device_id/baseline/capture",  post(baselines::post_capture))
        .route("/api/v1/devices/:device_id/baseline/history",  get(baselines::get_history))
        .route("/api/v1/events",                                get(events::list))
        .route("/api/v1/events/stream",                         get(events::stream))
        .route("/api/v1/events/:event_id",                      get(events::get_one))
        .route("/api/v1/events/ack",                            post(events::ack))
        .route("/api/v1/metrics/overhead",                      get(metrics::overhead))
        .route("/api/v1/mode",                                  get(mode::get_mode).post(mode::set_mode))
        .with_state(state)
}
