//! src/api/telemetry.rs - GET /api/v1/telemetry/latest, history, stream (SSE).

use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use axum::response::sse::{Event as SseEvent, Sse};
use futures_util::stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;

use super::SharedState;
use crate::storage::telemetry as db;

#[derive(Debug, Deserialize, Default)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
}

pub async fn latest(
    State(s): State<SharedState>,
    Path(sensor_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(cache) = &s.cache {
        if let Ok(Some(v)) = cache.get_last(&sensor_id).await {
            return Ok(Json(serde_json::json!({
                "sensor_id": sensor_id,
                "value": v,
                "source": "redis"
            })));
        }
    }
    if let Ok(Some(row)) = db::latest(s.pool.as_ref(), &sensor_id).await {
        return Ok(Json(serde_json::json!({
            "sensor_id": row.sensor_id,
            "value": row.value,
            "unit": row.unit,
            "ts": row.ts,
            "source": "timescaledb"
        })));
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn history(
    State(s): State<SharedState>,
    Path(sensor_id): Path<String>,
    Query(q): Query<HistoryQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = q.limit.unwrap_or(100).clamp(1, 1000);
    let rows = db::history(s.pool.as_ref(), &sensor_id, limit).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({
        "sensor_id": sensor_id,
        "limit": limit,
        "count": rows.len(),
        "rows": rows
    })))
}

pub async fn stream(
    State(_s): State<SharedState>,
) -> Sse<impl futures_util::Stream<Item = Result<SseEvent, Infallible>>> {
    let s = stream::unfold((), |_| async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        // Redis-stub: позже — Bus-подписка (P3.34, часть 2)
        let ev = SseEvent::default().event("telemetry").data("{}");
        Some((Ok(ev), ()))
    });
    Sse::new(s).keep_alive(
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))
    )
}
