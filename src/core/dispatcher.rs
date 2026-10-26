//! src/core/dispatcher.rs - Event routing across domains.
//!
//! Receives telemetry from Bus -> routes to 3 domains by priority.
//! On mode change: starts/stops corresponding domains.
//!
//! Key fixes vs v3.0:
//! - Receiver is NOT moved into domain: dispatcher owns it, domain gets
//!   messages via internal bounded channel.
//! - On mode change domain is not recreated from scratch - it receives
//!   DomainCommand::Stop, preserving its FSM state in core.

use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::{Mode, ModeState, Priority};

#[derive(Debug, Clone)]
pub enum DomainCommand {
    Start,
    Stop,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainKind {
    Hard,
    Soft,
    Life,
}

impl DomainKind {
    pub fn priority(self) -> Priority {
        match self {
            DomainKind::Hard => Priority::Hard,
            DomainKind::Soft => Priority::Soft,
            DomainKind::Life => Priority::Life,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            DomainKind::Hard => "hard",
            DomainKind::Soft => "soft",
            DomainKind::Life => "life",
        }
    }
    pub fn all() -> [DomainKind; 3] {
        [DomainKind::Hard, DomainKind::Soft, DomainKind::Life]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatcherError {
    #[error("bus closed")]
    BusClosed,
    #[error("domain channel closed: {0}")]
    DomainClosed(&'static str),
}

pub struct Dispatcher {
    mode: ModeState,
    bus: Bus,
    hard_tx: mpsc::Sender<BusMessage>,
    soft_tx: mpsc::Sender<BusMessage>,
    life_tx: mpsc::Sender<BusMessage>,
    cmd_tx: mpsc::Sender<(DomainKind, DomainCommand)>,
}

pub struct DomainReceivers {
    pub hard: mpsc::Receiver<BusMessage>,
    pub soft: mpsc::Receiver<BusMessage>,
    pub life: mpsc::Receiver<BusMessage>,
    pub commands: mpsc::Receiver<(DomainKind, DomainCommand)>,
}

impl Dispatcher {
    pub fn new(mode: ModeState, bus: Bus, domain_capacity: usize) -> (Self, DomainReceivers) {
        let cap = domain_capacity.max(16);
        let (hard_tx, hard_rx) = mpsc::channel(cap);
        let (soft_tx, soft_rx) = mpsc::channel(cap);
        let (life_tx, life_rx) = mpsc::channel(cap);
        let (cmd_tx, cmd_rx) = mpsc::channel(32);

        (
            Self {
                mode,
                bus,
                hard_tx,
                soft_tx,
                life_tx,
                cmd_tx,
            },
            DomainReceivers {
                hard: hard_rx,
                soft: soft_rx,
                life: life_rx,
                commands: cmd_rx,
            },
        )
    }

    async fn route(&self, msg: BusMessage) -> Result<(), DispatcherError> {
        let kind = match msg.priority {
            Priority::Hard => DomainKind::Hard,
            Priority::Soft => DomainKind::Soft,
            Priority::Life => DomainKind::Life,
        };
        let tx = match kind {
            DomainKind::Hard => &self.hard_tx,
            DomainKind::Soft => &self.soft_tx,
            DomainKind::Life => &self.life_tx,
        };
        tx.send(msg)
            .await
            .map_err(|_| DispatcherError::DomainClosed(kind.name()))
    }

    async fn apply_mode(&self, new_mode: Mode) -> Result<(), DispatcherError> {
        for kind in DomainKind::all() {
            let enabled = match kind {
                DomainKind::Hard => new_mode.hard_enabled(),
                DomainKind::Soft => new_mode.soft_enabled(),
                DomainKind::Life => new_mode.life_enabled(),
            };
            let cmd = if enabled {
                DomainCommand::Start
            } else {
                DomainCommand::Stop
            };
            self.cmd_tx
                .send((kind, cmd))
                .await
                .map_err(|_| DispatcherError::DomainClosed("commands"))?;
        }
        Ok(())
    }

    pub async fn run(
        self,
        mut telemetry_rx: mpsc::Receiver<BusMessage>,
        mut signals_rx: mpsc::Receiver<BusMessage>,
        mut status_rx: mpsc::Receiver<BusMessage>,
        mut diagnostics_rx: mpsc::Receiver<BusMessage>,
    ) -> Result<(), DispatcherError> {
        let initial = self.mode.get();
        info!(mode = initial.name(), "dispatcher starting");
        self.apply_mode(initial).await?;

        let mut mode_check = tokio::time::interval(std::time::Duration::from_millis(100));
        mode_check.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut last_mode = initial;

        loop {
            tokio::select! {
                Some(msg) = telemetry_rx.recv() => {
                    self.route(msg).await?;
                }
                Some(msg) = status_rx.recv() => {
                    let _ = self.bus.diagnostics.try_send(BusMessage::new(
                        "dispatcher",
                        Priority::Soft,
                        BusPayload::Diagnostic {
                            level: DiagLevel::Info,
                            component: msg.source.to_string(),
                            message: format!("status: {:?}", msg.payload),
                        },
                    ));
                }
                Some(msg) = diagnostics_rx.recv() => {
                    debug!(source = msg.source, "diag");
                }
                Some(msg) = signals_rx.recv() => {
                    match msg.payload {
                        BusPayload::ModeChange { target } => {
                            info!(to = target.name(), "mode change via signals");
                            self.mode.set(target);
                        }
                        BusPayload::Shutdown => {
                            info!("dispatcher shutdown");
                            break;
                        }
                        _ => {}
                    }
                }
                _ = mode_check.tick() => {
                    let cur = self.mode.get();
                    if cur != last_mode {
                        info!(from = last_mode.name(), to = cur.name(), "mode changed (tick)");
                        self.apply_mode(cur).await?;
                        last_mode = cur;
                    }
                }
            }
        }

        for kind in DomainKind::all() {
            let _ = self.cmd_tx.send((kind, DomainCommand::Shutdown)).await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::bus::{Bus, BusPayload};

    fn make() -> (Dispatcher, DomainReceivers, Bus) {
        let (bus, _rx) = Bus::new(16, 16, 16, 16);
        let mode = ModeState::with_mode(Mode::All);
        let (disp, drecv) = Dispatcher::new(mode, bus.clone(), 16);
        (disp, drecv, bus)
    }

    #[test]
    fn domain_kind_all_three() {
        assert_eq!(DomainKind::all().len(), 3);
    }

    #[test]
    fn domain_kind_priority_map() {
        assert_eq!(DomainKind::Hard.priority(), Priority::Hard);
        assert_eq!(DomainKind::Soft.priority(), Priority::Soft);
        assert_eq!(DomainKind::Life.priority(), Priority::Life);
    }

    #[tokio::test]
    async fn route_hard_to_hard() {
        let (disp, mut drecv, _bus) = make();
        let msg = BusMessage::new(
            "modbus",
            Priority::Hard,
            BusPayload::Telemetry {
                sensor_id: "s".into(),
                value: 1.0,
                unit: "".into(),
                timestamp_ms: 0,
            },
        );
        disp.route(msg).await.unwrap();
        let got = drecv.hard.recv().await.unwrap();
        assert_eq!(got.source, "modbus");
    }

    #[tokio::test]
    async fn route_soft_to_soft() {
        let (disp, mut drecv, _bus) = make();
        let msg = BusMessage::new(
            "matter",
            Priority::Soft,
            BusPayload::Telemetry {
                sensor_id: "s".into(),
                value: 2.0,
                unit: "".into(),
                timestamp_ms: 0,
            },
        );
        disp.route(msg).await.unwrap();
        assert!(drecv.soft.recv().await.is_some());
    }

    #[tokio::test]
    async fn route_life_to_life() {
        let (disp, mut drecv, _bus) = make();
        let msg = BusMessage::new(
            "mqtt",
            Priority::Life,
            BusPayload::Telemetry {
                sensor_id: "s".into(),
                value: 3.0,
                unit: "".into(),
                timestamp_ms: 0,
            },
        );
        disp.route(msg).await.unwrap();
        assert!(drecv.life.recv().await.is_some());
    }

    #[tokio::test]
    async fn apply_mode_sends_commands() {
        let (disp, mut drecv, _bus) = make();
        disp.apply_mode(Mode::HardOnly).await.unwrap();

        let mut got = vec![];
        for _ in 0..3 {
            got.push(drecv.commands.recv().await.unwrap());
        }
        let hard = got.iter().find(|(k, _)| *k == DomainKind::Hard).unwrap();
        let soft = got.iter().find(|(k, _)| *k == DomainKind::Soft).unwrap();
        let life = got.iter().find(|(k, _)| *k == DomainKind::Life).unwrap();
        assert!(matches!(hard.1, DomainCommand::Start));
        assert!(matches!(soft.1, DomainCommand::Stop));
        assert!(matches!(life.1, DomainCommand::Stop));
    }

    #[tokio::test]
    async fn apply_mode_all_enables_all() {
        let (disp, mut drecv, _bus) = make();
        disp.apply_mode(Mode::All).await.unwrap();
        let mut starts = 0;
        for _ in 0..3 {
            let (_, cmd) = drecv.commands.recv().await.unwrap();
            if matches!(cmd, DomainCommand::Start) {
                starts += 1;
            }
        }
        assert_eq!(starts, 3);
    }
}
