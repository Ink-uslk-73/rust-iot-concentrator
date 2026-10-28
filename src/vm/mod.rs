//! src/vm/mod.rs - Domain (VM): HARD/SOFT/LIFE/STORAGE.
//!
//! Each domain:
//! - receives BusMessage from internal channel (dispatcher)
//! - receives DomainCommand (Start/Stop/Shutdown)
//! - publishes result back to Bus (status/telemetry)
//!
//! On Stop: saves state (FSM) in saved_state, does not lose it.
//! On Start: restores state, continues.
//! On Shutdown: exits loop.

pub mod state;
pub mod hard;
pub mod life;
pub mod soft;
pub mod storage;

use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::dispatcher::{DomainCommand, DomainKind};
use crate::core::mode_switch::Priority;

pub trait Domain: Send {
    fn kind(&self) -> DomainKind;
    fn priority(&self) -> Priority;
    fn handle(&mut self, msg: BusMessage);
    fn tick(&mut self) {}
    fn snapshot(&self) -> Vec<u8> {
        Vec::new()
    }
    fn restore(&mut self, _state: &[u8]) {}
}

pub async fn run_domain<D: Domain>(
    mut domain: D,
    mut telemetry_rx: mpsc::Receiver<BusMessage>,
    mut commands_rx: mpsc::Receiver<(DomainKind, DomainCommand)>,
    bus: Bus,
    tick: std::time::Duration,
) {
    let kind = domain.kind();
    let priority = domain.priority();
    let mut running = true;
    let mut saved_state: Option<Vec<u8>> = None;
    let mut ticker = tokio::time::interval(tick);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    info!(domain = kind.name(), "domain started");

    loop {
        tokio::select! {
            Some(msg) = telemetry_rx.recv() => {
                if running {
                    domain.handle(msg);
                } else {
                    debug!(domain = kind.name(), "msg dropped (stopped)");
                }
            }
            Some((target, cmd)) = commands_rx.recv() => {
                if target != kind {
                    continue;
                }
                match cmd {
                    DomainCommand::Start => {
                        if !running {
                            if let Some(s) = saved_state.take() {
                                domain.restore(&s);
                            }
                            running = true;
                            info!(domain = kind.name(), "started");
                            let _ = bus.status.try_send(BusMessage::new(
                                "vm",
                                priority,
                                BusPayload::Status {
                                    component: kind.name().into(),
                                    state: "running".into(),
                                    detail: String::new(),
                                },
                            ));
                        }
                    }
                    DomainCommand::Stop => {
                        if running {
                            saved_state = Some(domain.snapshot());
                            running = false;
                            info!(domain = kind.name(), "stopped");
                            let _ = bus.status.try_send(BusMessage::new(
                                "vm",
                                priority,
                                BusPayload::Status {
                                    component: kind.name().into(),
                                    state: "stopped".into(),
                                    detail: String::new(),
                                },
                            ));
                        }
                    }
                    DomainCommand::Shutdown => {
                        info!(domain = kind.name(), "shutdown");
                        let _ = bus.diagnostics.try_send(BusMessage::new(
                            "vm",
                            priority,
                            BusPayload::Diagnostic {
                                level: DiagLevel::Info,
                                component: kind.name().into(),
                                message: "shutdown".into(),
                            },
                        ));
                        return;
                    }
                }
            }
            _ = ticker.tick() => {
                if running {
                    domain.tick();
                }
            }
            else => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDomain {
        kind: DomainKind,
        handled: usize,
        state: Vec<u8>,
    }
    impl Domain for TestDomain {
        fn kind(&self) -> DomainKind { self.kind }
        fn priority(&self) -> Priority { self.kind.priority() }
        fn handle(&mut self, _msg: BusMessage) { self.handled += 1; }
        fn snapshot(&self) -> Vec<u8> { self.state.clone() }
        fn restore(&mut self, s: &[u8]) { self.state = s.to_vec(); }
    }

    #[tokio::test]
    async fn handles_messages_while_running() {
        let (bus, _rx) = Bus::new(64, 64, 64, 64, std::sync::Arc::new(crate::metrics::OverheadMetrics::new()));
        let (tele_tx, tele_rx) = mpsc::channel(16);
        let (cmd_tx, cmd_rx) = mpsc::channel(16);
        let d = TestDomain {
            kind: DomainKind::Hard,
            handled: 0,
            state: vec![],
        };
        let h = tokio::spawn(run_domain(
            d,
            tele_rx,
            cmd_rx,
            bus,
            std::time::Duration::from_millis(50),
        ));
        tele_tx
            .send(BusMessage::new(
                "modbus",
                Priority::Hard,
                BusPayload::Telemetry {
                    sensor_id: "s".into(),
                    value: 1.0,
                    unit: "".into(),
                    timestamp_ms: 0,
                },
            ))
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        let _ = cmd_tx
            .send((DomainKind::Hard, DomainCommand::Shutdown))
            .await;
        let _ = tokio::time::timeout(std::time::Duration::from_secs(1), h).await;
    }

    #[tokio::test]
    async fn ignores_commands_for_other_domains() {
        let (bus, _rx) = Bus::new(64, 64, 64, 64, std::sync::Arc::new(crate::metrics::OverheadMetrics::new()));
        let (_tele_tx, tele_rx) = mpsc::channel(16);
        let (cmd_tx, cmd_rx) = mpsc::channel(16);
        let d = TestDomain {
            kind: DomainKind::Life,
            handled: 0,
            state: vec![],
        };
        let h = tokio::spawn(run_domain(
            d,
            tele_rx,
            cmd_rx,
            bus,
            std::time::Duration::from_millis(50),
        ));
        cmd_tx.send((DomainKind::Hard, DomainCommand::Shutdown)).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        cmd_tx.send((DomainKind::Life, DomainCommand::Shutdown)).await.unwrap();
        assert!(tokio::time::timeout(std::time::Duration::from_secs(1), h).await.is_ok());
    }
}
pub async fn run_domain_with_sem<D: crate::vm::Domain>(
    domain: D,
    telemetry_rx: tokio::sync::mpsc::Receiver<crate::core::bus::BusMessage>,
    commands_rx: tokio::sync::mpsc::Receiver<(crate::core::dispatcher::DomainKind, crate::core::dispatcher::DomainCommand)>,
    bus: crate::core::bus::Bus,
    tick: std::time::Duration,
    sem: crate::vm::state::VmSemaphore,
) {
    use crate::vm::state::VmState;
    sem.set(VmState::Waiting);
    sem.set(VmState::Running);
    run_domain(domain, telemetry_rx, commands_rx, bus, tick).await;
    sem.set(VmState::Stopped);
}