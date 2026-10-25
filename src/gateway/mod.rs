//! src/gateway/mod.rs - TLS gateway for tablet (mTLS + axum).
pub mod tls;

use std::path::PathBuf;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("tls: {0}")]
    Tls(String),
    #[error("toml: {0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub terminal: TerminalConfig,
    pub audit: AuditConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub dashboard_addr: String,
    pub terminal_addr: String,
    pub require_mtls: bool,
    #[serde(default = "default_tls_version")]
    pub min_tls_version: String,
}

fn default_tls_version() -> String {
    "1.3".into()
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub cert: PathBuf,
    pub key: PathBuf,
    pub ca: PathBuf,
    pub client_cert_required: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TerminalConfig {
    pub allowed_domains: Vec<String>,
    pub allowed_commands: Vec<String>,
    pub deny_shell: bool,
    pub audit_file: PathBuf,
    pub session_timeout_sec: u64,
    pub max_concurrent: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuditConfig {
    pub enabled: bool,
    pub file: PathBuf,
    pub to_diagnostics: bool,
}

impl GatewayConfig {
    pub fn load(path: &str) -> Result<Self, GatewayError> {
        let s = std::fs::read_to_string(path)?;
        let cfg: GatewayConfig = toml::from_str(&s)?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<(), GatewayError> {
        if self.server.require_mtls != self.tls.client_cert_required {
            return Err(GatewayError::Tls(
                "server.require_mtls and tls.client_cert_required must agree".into(),
            ));
        }
        if self.server.min_tls_version != "1.3" && self.server.min_tls_version != "1.2" {
            return Err(GatewayError::Tls(format!(
                "min_tls_version must be 1.2 or 1.3, got {}",
                self.server.min_tls_version
            )));
        }
        if self.terminal.deny_shell && self.terminal.allowed_commands.is_empty() {
            return Err(GatewayError::Tls(
                "deny_shell=true requires non-empty allowed_commands".into(),
            ));
        }
        if self.audit.enabled && self.audit.file != self.terminal.audit_file {
            return Err(GatewayError::Tls(
                "audit.file and terminal.audit_file must match".into(),
            ));
        }
        Ok(())
    }
}
