//! src/core/bus.rs - Shared bus: 4 channels by signal type.
//!
//! Not by priority (HARD/SOFT/LIFE), but by TYPE:
//! - signals     - control commands (from tablet, from FSM)
//! - telemetry   - data (read-only, from protocols)
//! - status      - component state (from supervisor, watchdog)
//! - diagnostics - errors, emergency states

use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use thiserror::Error;

use crate::core::mode_switch::{Mode, Priority};

static SEQ: AtomicU64 = AtomicU64::new(1);

pub fn next_seq() -> u64 {
    SEQ.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagLevel {
    Info,
    Warning,
    Critical,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusPayload {
    Command {
        target: String,
        action: String,
        params: serde_json::Value,
    },
    Telemetry {
        sensor_id: String,
        value: f64,
        unit: String,
        timestamp_ms: u64,
    },
    Status {
        component: String,
        state: String,
        detail: String,
    },
    Diagnostic {
        level: DiagLevel,
        component: String,
        message: String,
    },
    ModeChange { target: Mode },
    Shutdown,
}

impl BusPayload {
    pub fn channel(&self) -> Channel {
        match self {
            BusPayload::Command { .. }
            | BusPayload::ModeChange { .. }
            | BusPayload::Shutdown => Channel::Signals,
            BusPayload::Telemetry { .. } => Channel::Telemetry,
            BusPayload::Status { .. } => Channel::Status,
            BusPayload::Diagnostic { .. } => Channel::Diagnostics,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Signals,
    Telemetry,
    Status,
    Diagnostics,
}

#[derive(Debug, Clone)]
pub struct BusMessage {
    pub seq: u64,
    pub source: &'static str,
    pub priority: Priority,
    pub payload: BusPayload,
}

impl BusMessage {
    pub fn new(source: &'static str, priority: Priority, payload: BusPayload) -> Self {
        Self { seq: next_seq(), source, priority, payload }
    }
}

#[derive(Debug, Error)]
pub enum BusError {
    #[error("channel closed: {0:?}")]
    Closed(Channel),
    #[error("channel full: {0:?}")]
    Full(Channel),
}

#[derive(Clone)]
pub struct Bus {
    pub signals: mpsc::Sender<BusMessage>,
    pub telemetry: mpsc::Sender<BusMessage>,
    pub status: mpsc::Sender<BusMessage>,
    pub diagnostics: mpsc::Sender<BusMessage>,
}

impl Bus {
    pub fn new(
        signals_cap: usize,
        telemetry_cap: usize,
        status_cap: usize,
        diagnostics_cap: usize,
    ) -> (Self, BusReceivers) {
        let (signals_tx, signals_rx) = mpsc::channel(signals_cap.max(1));
        let (telemetry_tx, telemetry_rx) = mpsc::channel(telemetry_cap.max(1));
        let (status_tx, status_rx) = mpsc::channel(status_cap.max(1));
        let (diag_tx, diag_rx) = mpsc::channel(diagnostics_cap.max(1));
        (
            Self {
                signals: signals_tx,
                telemetry: telemetry_tx,
                status: status_tx,
                diagnostics: diag_tx,
            },
            BusReceivers {
                signals: signals_rx,
                telemetry: telemetry_rx,
                status: status_rx,
                diagnostics: diag_rx,
            },
        )
    }

    pub async fn send(&self, msg: BusMessage) -> Result<(), BusError> {
        let ch = msg.payload.channel();
        let tx = match ch {
            Channel::Signals => &self.signals,
            Channel::Telemetry => &self.telemetry,
            Channel::Status => &self.status,
            Channel::Diagnostics => &self.diagnostics,
        };
        tx.send(msg).await.map_err(|_| BusError::Closed(ch))
    }

    pub fn try_send(&self, msg: BusMessage) -> Result<(), BusError> {
        let ch = msg.payload.channel();
        let tx = match ch {
            Channel::Signals => &self.signals,
            Channel::Telemetry => &self.telemetry,
            Channel::Status => &self.status,
            Channel::Diagnostics => &self.diagnostics,
        };
        match tx.try_send(msg) {
            Ok(()) => Ok(()),
            Err(mpsc::error::TrySendError::Full(_)) => Err(BusError::Full(ch)),
            Err(mpsc::error::TrySendError::Closed(_)) => Err(BusError::Closed(ch)),
        }
    }
}

pub struct BusReceivers {
    pub signals: mpsc::Receiver<BusMessage>,
    pub telemetry: mpsc::Receiver<BusMessage>,
    pub status: mpsc::Receiver<BusMessage>,
    pub diagnostics: mpsc::Receiver<BusMessage>,
}

impl BusReceivers {
    pub fn split(self) -> (
        mpsc::Receiver<BusMessage>,
        mpsc::Receiver<BusMessage>,
        mpsc::Receiver<BusMessage>,
        mpsc::Receiver<BusMessage>,
    ) {
        (self.signals, self.telemetry, self.status, self.diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_bus() -> (Bus, BusReceivers) {
        Bus::new(4, 8, 4, 4)
    }

    #[test]
    fn payload_routes_to_correct_channel() {
        assert_eq!(
            BusPayload::Command {
                target: "x".into(),
                action: "y".into(),
                params: serde_json::json!({})
            }
            .channel(),
            Channel::Signals
        );
        assert_eq!(
            BusPayload::Telemetry {
                sensor_id: "s".into(),
                value: 1.0,
                unit: "C".into(),
                timestamp_ms: 0
            }
            .channel(),
            Channel::Telemetry
        );
        assert_eq!(
            BusPayload::Status {
                component: "c".into(),
                state: "ok".into(),
                detail: "".into()
            }
            .channel(),
            Channel::Status
        );
        assert_eq!(
            BusPayload::Diagnostic {
                level: DiagLevel::Info,
                component: "c".into(),
                message: "m".into()
            }
            .channel(),
            Channel::Diagnostics
        );
        assert_eq!(BusPayload::Shutdown.channel(), Channel::Signals);
        assert_eq!(
            BusPayload::ModeChange { target: Mode::All }.channel(),
            Channel::Signals
        );
    }

    #[tokio::test]
    async fn send_routes_telemetry() {
        let (bus, mut rx) = make_bus();
        let msg = BusMessage::new(
            "modbus",
            Priority::Hard,
            BusPayload::Telemetry {
                sensor_id: "s1".into(),
                value: 42.0,
                unit: "C".into(),
                timestamp_ms: 1,
            },
        );
        bus.send(msg).await.unwrap();
        let got = rx.telemetry.recv().await.unwrap();
        assert_eq!(got.source, "modbus");
        assert_eq!(got.priority, Priority::Hard);
    }

    #[tokio::test]
    async fn send_routes_signals() {
        let (bus, mut rx) = make_bus();
        let msg = BusMessage::new("core", Priority::Hard, BusPayload::Shutdown);
        bus.send(msg).await.unwrap();
        let got = rx.signals.recv().await.unwrap();
        assert!(matches!(got.payload, BusPayload::Shutdown));
    }

    #[tokio::test]
    async fn send_closed_returns_error() {
        let (bus, rx) = make_bus();
        drop(rx);
        let msg = BusMessage::new("x", Priority::Hard, BusPayload::Shutdown);
        let err = bus.send(msg).await.unwrap_err();
        assert!(matches!(err, BusError::Closed(Channel::Signals)));
    }

    #[tokio::test]
    async fn try_send_full_returns_error() {
        let (bus, _rx) = Bus::new(1, 1, 1, 1);
        bus.try_send(BusMessage::new("a", Priority::Hard, BusPayload::Shutdown))
            .unwrap();
        let err = bus
            .try_send(BusMessage::new("b", Priority::Hard, BusPayload::Shutdown))
            .unwrap_err();
        assert!(matches!(err, BusError::Full(Channel::Signals)));
    }

    #[test]
    fn seq_is_monotonic() {
        let a = next_seq();
        let b = next_seq();
        assert!(b > a);
    }
}
