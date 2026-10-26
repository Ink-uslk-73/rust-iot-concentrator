//! src/aig/mod.rs - AIG-502 REST integration.
//!
//! Modules:
//! - client  : HTTP client to thingspro-api-v1 (config.json / twin)
//! - drift   : compare live state with baseline
//! - attck   : map AIG events to ATT&CK for ICS techniques
//! - poller  : periodic poll, drift -> event -> Bus

pub mod attck;
pub mod client;
pub mod drift;
pub mod poller;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AigError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("config: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AigError>;
