//! src/api/mode.rs - mode get/set endpoint.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::core::mode_switch::{Mode, Priority};
use crate::core::bus::{BusMessage, BusPayload};
use super::SharedState;

#[derive(Debug, Serialize)]
pub struct ModeResponse {
    pub mode: String,
    pub hard: bool,
    pub soft: bool,
    pub life: bool,
}

impl From<Mode> for ModeResponse {
    fn from(m: Mode) -> Self {
        Self {
            mode: m.name().to_string(),
            hard: m.hard_enabled(),
            soft: m.soft_enabled(),
            life: m.life_enabled(),
        }
    }
}

pub async fn get_mode(State(s): State<SharedState>) -> Json<ModeResponse> {
    Json(s.mode.get().into())
}

#[derive(Debug, Deserialize)]
pub struct SetModeRequest {
    pub mode: String,
    pub operator: Option<String>,
}

pub async fn set_mode(
    State(s): State<SharedState>,
    Json(req): Json<SetModeRequest>,
) -> Result<Json<ModeResponse>, (StatusCode, String)> {
    let new_mode = Mode::from_name(&req.mode)
        .ok_or((StatusCode::BAD_REQUEST, format!("unknown mode: {}", req.mode)))?;
    let prev = s.mode.set(new_mode);
    tracing::info!(
        operator = ?req.operator,
        from = prev.name(),
        to = new_mode.name(),
        "mode change via API"
    );
    // Notify bus (dispatcher will react)
    let _ = s.bus.send(BusMessage::new(
        "api",
        Priority::Hard,
        BusPayload::ModeChange { target: new_mode },
    )).await;
    Ok(Json(new_mode.into()))
}
