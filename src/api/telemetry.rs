//! src/api/telemetry.rs - GET /api/v1/telemetry/latest (Redis first).

use axum::{extract::{Path, State}, http::StatusCode, Json};

use super::SharedState;

pub async fn latest(
    State(s): State<SharedState>,
    Path(sensor_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(cache) = &s.cache {
        match cache.get_last(&sensor_id).await {
            Ok(Some(v)) => return Ok(Json(serde_json::json!({
                "sensor_id": sensor_id,
                "value": v,
                "source": "redis"
            }))),
            _ => {}
        }
    }
    // fallback: TimescaleDB (last row) - not implemented yet
    Err(StatusCode::NOT_FOUND)
}

