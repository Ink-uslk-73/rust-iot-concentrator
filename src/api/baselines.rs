//! src/api/baselines.rs - baseline REST handlers.

use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use std::time::Instant;

use crate::storage::baselines::{self, Baseline, BaselineSummary};
use super::SharedState;

pub async fn get_latest(
    State(s): State<SharedState>,
    Path(device_id): Path<String>,
) -> Result<Json<Baseline>, StatusCode> {
    let t = Instant::now();
    let res = match baselines::latest(s.pool.as_ref(), &s.root, &device_id).await {
        Ok(Some(b)) => Ok(Json(b)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    s.metrics.record_api_latency(t);
    res
}

pub async fn get_summary(
    State(s): State<SharedState>,
    Path(device_id): Path<String>,
) -> Result<Json<BaselineSummary>, StatusCode> {
    let t = Instant::now();
    let res = match baselines::latest(s.pool.as_ref(), &s.root, &device_id).await {
        Ok(Some(b)) => {
            let critical = b.payload.get("critical_paths")
                .and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
            let high = b.payload.get("high_paths")
                .and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
            Ok(Json(BaselineSummary {
                device_id: b.device_id.clone(),
                baseline_id: baselines::make_id(&b.device_id, b.version),
                version: b.version,
                captured_at: b.captured_at,
                hash: b.hash,
                critical_paths_count: critical,
                high_paths_count: high,
                drift_status: "unknown".into(),
            }))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    s.metrics.record_api_latency(t);
    res
}

pub async fn get_diff(
    State(s): State<SharedState>,
    Path(device_id): Path<String>,
) -> Json<serde_json::Value> {
    let _ = (s, device_id);
    Json(serde_json::json!({ "diffs": [] }))
}

#[derive(Debug, Deserialize)]
pub struct CaptureRequest {
    pub operator: String,
    pub reason: String,
    pub approval_ticket: Option<String>,
    pub payload: serde_json::Value,
}

pub async fn post_capture(
    State(s): State<SharedState>,
    Path(device_id): Path<String>,
    Json(req): Json<CaptureRequest>,
) -> Result<Json<Baseline>, StatusCode> {
    let t = Instant::now();
    let res = async {
        let Some(ticket) = req.approval_ticket.clone() else {
            return Err(StatusCode::FORBIDDEN);
        };
        let existing = baselines::history(s.pool.as_ref(), &s.root, &device_id, 1000)
            .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let version = baselines::next_version(&existing);
        let hash = baselines::sha256_hex(&serde_json::to_vec(&req.payload).unwrap_or_default());
        let b = Baseline {
            device_id: device_id.clone(),
            version,
            hash,
            captured_at: chrono::Utc::now(),
            operator: req.operator,
            reason: req.reason,
            approval_ticket: Some(ticket),
            payload: req.payload,
            git_commit: None,
        };
        baselines::save(s.pool.as_ref(), &s.root, &b)
            .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(b))
    }.await;
    s.metrics.record_api_latency(t);
    res
}

pub async fn get_history(
    State(s): State<SharedState>,
    Path(device_id): Path<String>,
) -> Result<Json<Vec<Baseline>>, StatusCode> {
    let t = Instant::now();
    let res = baselines::history(s.pool.as_ref(), &s.root, &device_id, 100)
        .await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    s.metrics.record_api_latency(t);
    res
}
