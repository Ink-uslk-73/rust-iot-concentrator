//! src/api/events.rs - events REST + SSE handlers.

use axum::{extract::{Path, Query, State}, http::StatusCode, response::{sse::{Event as SseEvent, Sse}, IntoResponse}, Json};
use chrono::{DateTime, Utc};
use futures_util::stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;

use crate::storage::events::{self, Event, EventFilter};
use super::SharedState;

#[derive(Debug, Deserialize, Default)]
pub struct ListQuery {
    pub device_id: Option<String>,
    pub severity: Option<String>,
    pub technique: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
}

pub async fn list(
    State(s): State<SharedState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<Event>>, StatusCode> {
    let f = EventFilter {
        device_id: q.device_id,
        severity: q.severity,
        technique: q.technique,
        since: q.since,
        limit: q.limit,
    };
    events::list(s.pool.as_ref(), &f).await
        .map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn stream(
    State(s): State<SharedState>,
) -> impl IntoResponse {
    let state = s.clone();
    let stream = stream::unfold(state, |state| async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let f = EventFilter { limit: Some(50), ..Default::default() };
        let events = events::list(state.pool.as_ref(), &f).await.unwrap_or_default();
        let payload = serde_json::to_string(&events).unwrap_or_default();
        let ev = SseEvent::default().event("events").data(payload);
        Some((Ok::<_, Infallible>(ev), state))
    });
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))
    )
}

pub async fn get_one(
    State(s): State<SharedState>,
    Path(event_id): Path<String>,
) -> Result<Json<Event>, StatusCode> {
    match events::get_one(s.pool.as_ref(), &event_id).await {
        Ok(Some(e)) => Ok(Json(e)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Deserialize)]
pub struct AckRequest {
    pub event_id: String,
    pub by: String,
}

pub async fn ack(
    State(s): State<SharedState>,
    Json(req): Json<AckRequest>,
) -> Result<StatusCode, StatusCode> {
    events::ack(s.pool.as_ref(), &req.event_id, &req.by).await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
