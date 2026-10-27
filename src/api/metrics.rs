//! src/api/metrics.rs - overhead metrics endpoint.

use axum::{extract::State, Json};

use super::SharedState;

pub async fn overhead(
    State(s): State<SharedState>,
) -> Json<serde_json::Value> {
    let m = s.metrics.snapshot();
    let status = match m.status {
        crate::metrics::OverheadStatus::Ok => "ok",
        crate::metrics::OverheadStatus::Degraded => "degraded",
        crate::metrics::OverheadStatus::Critical => "critical",
    };
    Json(serde_json::json!({
        "hard_latency_us": m.hard_latency_us,
        "api_latency_us": m.api_latency_us,
        "sse_jitter_ms": m.sse_jitter_ms,
        "bus_buffer_pct": m.bus_buffer_pct,
        "bus_drops": m.bus_drops,
        "sse_retransmits": m.sse_retransmits,
        "thresholds": {
            "hard_latency_us_max": 1000,
            "api_latency_us_max": 1000,
            "sse_jitter_ms_max": 10,
            "bus_buffer_pct_max": 70
        },
        "status": status
    }))
}
