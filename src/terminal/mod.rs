//! src/terminal/mod.rs - terminal for tablet (WebSocket <-> PTY/SSH).
//!
//! Decisions (module 1, section 1):
//! - WebSocket proxy, not direct SSH.
//! - Allowlist of commands/domains.
//! - Audit of each session to logs/terminal-audit.log + diagnostics.
//! - PTY via portable-pty; SSH via russh (module 7).

pub mod allowlist;
pub mod audit;
pub mod pty;

use thiserror::Error;
use uuid::Uuid;

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;

#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("pty: {0}")]
    Pty(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("denied: {0}")]
    Denied(String),
}

pub struct TerminalSession {
    pub id: Uuid,
    pub domain: String,
    pub tablet_cn: String,
    pub bus: Bus,
}

impl TerminalSession {
    pub fn new(domain: impl Into<String>, tablet_cn: impl Into<String>, bus: Bus) -> Self {
        Self {
            id: Uuid::new_v4(),
            domain: domain.into(),
            tablet_cn: tablet_cn.into(),
            bus,
        }
    }

    pub fn audit(&self, event: &str, command: Option<&str>, result: &str) {
        let entry = serde_json::json!({
            "ts": now_rfc3339(),
            "tablet_cert": self.tablet_cn,
            "domain": self.domain,
            "session_id": self.id,
            "event": event,
            "command": command,
            "result": result,
        });
        audit::append(&entry);
        let _ = self.bus.diagnostics.try_send(BusMessage::new(
            "terminal",
            Priority::Life,
            BusPayload::Diagnostic {
                level: DiagLevel::Info,
                component: "terminal".into(),
                message: entry.to_string(),
            },
        ));
    }
}

fn now_rfc3339() -> String {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}.{:03}Z", d.as_secs(), d.subsec_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_audit_writes_diagnostics() {
        let (bus, mut rx) = Bus::new(64, 64, 64, 64, std::sync::Arc::new(crate::metrics::OverheadMetrics::new()));
        let s = TerminalSession::new("vm-hard", "CN=tablet-01", bus);
        s.audit("open", None, "ok");
        let m = rx.diagnostics.try_recv().unwrap();
        assert_eq!(m.source, "terminal");
        assert!(matches!(m.payload, BusPayload::Diagnostic { .. }));
    }

    #[test]
    fn session_id_is_uuid() {
        let (bus, _) = Bus::new(1, 1, 1, 1, std::sync::Arc::new(crate::metrics::OverheadMetrics::new()));
        let s = TerminalSession::new("core", "x", bus);
        assert!(!s.id.is_nil());
    }
}
