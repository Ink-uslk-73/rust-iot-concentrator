//! src/core/bus.rs - Shared bus: 4 channels by signal type.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use thiserror::Error;

use crate::core::mode_switch::{Mode, Priority};
use crate::metrics::OverheadMetrics;

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
    Command { target: String, action: String, params: serde_json::Value },
    Telemetry { sensor_id: String, value: f64, unit: String, timestamp_ms: u64 },
    Status { component: String, state: String, detail: String },
    Diagnostic { level: DiagLevel, component: String, message: String },
    ModeChange { target: Mode },
    Shutdown,
}

impl BusPayload {
    pub fn channel(&self) -> Channel {
        match self {
            BusPayload::Command { .. } | BusPayload::ModeChange { .. } | BusPayload::Shutdown => Channel::Signals,
            BusPayload::Telemetry { .. } => Channel::Telemetry,
            BusPayload::Status { .. } => Channel::Status,
            BusPayload::Diagnostic { .. } => Channel::Diagnostics,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel { Signals, Telemetry, Status, Diagnostics }

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
    pub metrics: Arc<OverheadMetrics>,
}

impl Bus {
    pub fn new(
        signals_cap: usize,
        telemetry_cap: usize,
        status_cap: usize,
        diagnostics_cap: usize,
        metrics: Arc<OverheadMetrics>,
    ) -> (Self, BusReceivers) {
        let (signals_tx, signals_rx) = mpsc::channel(signals_cap.max(1));
        let (telemetry_tx, telemetry_rx) = mpsc::channel(telemetry_cap.max(1));
        let (status_tx, status_rx) = mpsc::channel(status_cap.max(1));
        let (diag_tx, diag_rx) = mpsc::channel(diagnostics_cap.max(1));
        (
            Self {
                signals: signals_tx, telemetry: telemetry_tx,
                status: status_tx, diagnostics: diag_tx,
                metrics,
            },
            BusReceivers { signals: signals_rx, telemetry: telemetry_rx, status: status_rx, diagnostics: diag_rx },
        )
    }

    /// Per-priority send:
    /// - Hard: try_send (не блокировать; drop-on-full);
    /// - Soft/Life: send с timeout (HARD: 1ms, SOFT: 10ms, LIFE: 100ms).
    pub async fn send(&self, msg: BusMessage) -> Result<(), BusError> {
        let ch = msg.payload.channel();
        let tx = match ch {
            Channel::Signals => &self.signals,
            Channel::Telemetry => &self.telemetry,
            Channel::Status => &self.status,
            Channel::Diagnostics => &self.diagnostics,
        };
        let capacity = tx.max_capacity();

        let res = match msg.priority {
            Priority::Hard => {
                // HARD: никогда не блокировать
                tx.try_send(msg).map_err(|e| match e {
                    mpsc::error::TrySendError::Full(_) => BusError::Full(ch),
                    mpsc::error::TrySendError::Closed(_) => BusError::Closed(ch),
                })
            }
            Priority::Soft | Priority::Life => {
                let timeout = match msg.priority {
                    Priority::Soft => std::time::Duration::from_millis(10),
                    Priority::Life => std::time::Duration::from_millis(100),
                    Priority::Hard => unreachable!(),
                };
                match tokio::time::timeout(timeout, tx.send(msg)).await {
                    Ok(Ok(())) => Ok(()),
                    Ok(Err(_)) => Err(BusError::Closed(ch)),
                    Err(_) => Err(BusError::Full(ch)),
                }
            }
        };

        if res.is_err() {
            self.metrics.record_bus_drop();
        }
        self.metrics.record_bus_buffer(capacity, tx.capacity());
        res
    }

    pub fn try_send(&self, msg: BusMessage) -> Result<(), BusError> {
        let ch = msg.payload.channel();
        let tx = match ch {
            Channel::Signals => &self.signals,
            Channel::Telemetry => &self.telemetry,
            Channel::Status => &self.status,
            Channel::Diagnostics => &self.diagnostics,
        };
        let capacity = tx.max_capacity();
        let res = match tx.try_send(msg) {
            Ok(()) => Ok(()),
            Err(mpsc::error::TrySendError::Full(_)) => Err(BusError::Full(ch)),
            Err(mpsc::error::TrySendError::Closed(_)) => Err(BusError::Closed(ch)),
        };
        if res.is_err() {
            self.metrics.record_bus_drop();
        }
        self.metrics.record_bus_buffer(capacity, tx.capacity());
        res
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
        let m = Arc::new(OverheadMetrics::new());
        Bus::new(4, 8, 4, 4, m)
    }

    #[tokio::test]
    async fn send_routes_telemetry() {
        let (bus, mut rx) = make_bus();
        bus.send(BusMessage::new("modbus", Priority::Hard, BusPayload::Telemetry {
            sensor_id: "s1".into(), value: 42.0, unit: "C".into(), timestamp_ms: 1,
        })).await.unwrap();
        let got = rx.telemetry.recv().await.unwrap();
        assert_eq!(got.source, "modbus");
    }

    #[tokio::test]
    async fn bus_buffer_recorded() {
        let (bus, _rx) = make_bus();
        bus.send(BusMessage::new("x", Priority::Hard, BusPayload::Shutdown)).await.unwrap();
        let s = bus.metrics.snapshot();
        assert!(s.bus_buffer_pct <= 100);
    }
}
