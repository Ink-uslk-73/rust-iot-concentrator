//! src/api/satellites.rs - GET /api/v1/satellites.

use axum::{extract::State, Json};
use std::time::Duration;
use super::SharedState;

pub async fn list(State(s): State<SharedState>) -> Json<Vec<crate::satellite::SatelliteStatus>> {
    Json(s.satellites.check(Duration::from_secs(10), 3))
}
