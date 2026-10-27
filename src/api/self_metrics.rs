//! src/api/self_metrics.rs - self-diagnostics endpoint.

use axum::{extract::State, Json};

use super::SharedState;

pub async fn self_metrics(
    State(s): State<SharedState>,
) -> Json<serde_json::Value> {
    let g = s.registry.gauges_snapshot();
    let self_only: std::collections::HashMap<_, _> = g
        .into_iter()
        .filter(|(k, _)| k.starts_with("self."))
        .collect();
    Json(serde_json::json!(self_only))
}
