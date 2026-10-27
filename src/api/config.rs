//! src/api/config.rs - config mode switch (production / emulation).

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use super::SharedState;

#[derive(Debug, Serialize)]
pub struct ConfigModeResponse {
    pub mode: String,
    pub config_path: String,
    pub registers_path: String,
}

pub async fn get_config_mode(
    State(s): State<SharedState>,
) -> Json<ConfigModeResponse> {
    let cfg = s.config.read().unwrap();
    let (c, r) = cfg.modbus.current_paths();
    Json(ConfigModeResponse {
        mode: cfg.modbus.mode.clone(),
        config_path: c,
        registers_path: r,
    })
}

#[derive(Debug, Deserialize)]
pub struct SetConfigModeRequest {
    pub mode: String,
}

pub async fn set_config_mode(
    State(s): State<SharedState>,
    Json(req): Json<SetConfigModeRequest>,
) -> Result<Json<ConfigModeResponse>, (StatusCode, String)> {
    if req.mode != "production" && req.mode != "emulation" {
        return Err((StatusCode::BAD_REQUEST, format!("unknown mode: {}", req.mode)));
    }
    let mut cfg = s.config.write().unwrap();
    cfg.modbus.mode = req.mode.clone();
    let (c, r) = cfg.modbus.current_paths();
    tracing::info!(mode = %req.mode, "config mode changed via API");
    Ok(Json(ConfigModeResponse {
        mode: req.mode,
        config_path: c,
        registers_path: r,
    }))
}
