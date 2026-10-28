//! src/supervisor.rs - Supervision tree (JoinSet, CancellationToken, let-it-crash).

use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::vm::state::{VmSemaphore, VmState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorState { Running, Restarting, Stopped }

pub struct Supervisor {
    token: CancellationToken,
    set: JoinSet<()>,
    semaphores: Vec<(String, VmSemaphore)>,
    bus: Option<Bus>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            set: JoinSet::new(),
            semaphores: Vec::new(),
            bus: None,
        }
    }

    pub fn with_bus(mut self, bus: Bus) -> Self {
        self.bus = Some(bus);
        self
    }

    pub fn register(&mut self, name: &str, sem: VmSemaphore) {
        self.semaphores.push((name.to_string(), sem));
    }

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.set.spawn(fut);
    }

    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                _ = self.token.cancelled() => {
                    info!("supervisor: cancelled");
                    break;
                }
                Some(res) = self.set.join_next() => {
                    if let Err(e) = res {
                        warn!("supervisor: task panicked: {}", e);
                        if let Some(bus) = &self.bus {
                            let _ = bus.diagnostics.try_send(BusMessage::new(
                                "supervisor",
                                Priority::Hard,
                                BusPayload::Diagnostic {
                                    level: DiagLevel::Critical,
                                    component: "supervisor".into(),
                                    message: format!("task panicked: {}", e),
                                },
                            ));
                        }
                    }
                }
                else => break,
            }
        }
        self.set.shutdown().await;
        info!("supervisor: shutdown complete");
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }
}

impl Default for Supervisor {
    fn default() -> Self { Self::new() }
}

pub fn set_state(sem: &VmSemaphore, s: VmState) {
    sem.set(s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn supervisor_spawns_and_cancels() {
        let mut sup = Supervisor::new();
        let token = sup.token();
        sup.spawn(async move {
            token.cancelled().await;
        });
        let t = sup.token();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            t.cancel();
        });
        sup.run().await;
    }
}
impl Supervisor {
    /// Задача с автоперезапуском: фабрика вызывается заново при каждой панике.
    /// ВАЖНО: не использовать для задач с уникальными ресурсами (mpsc::Receiver).
    pub fn spawn_restartable<F, Fut>(&mut self, name: &'static str, factory: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        use futures_util::FutureExt;
        let factory = std::sync::Arc::new(factory);
        let bus = self.bus.clone();
        let token = self.token.clone();
        self.set.spawn(async move {
            loop {
                if token.is_cancelled() { break; }
                let fut = factory();
                let result = std::panic::AssertUnwindSafe(fut).catch_unwind().await;
                match result {
                    Ok(()) => break,
                    Err(_) => {
                        warn!("supervisor: task '{}' panicked, restart in 1s", name);
                        if let Some(bus) = &bus {
                            let _ = bus.diagnostics.try_send(BusMessage::new(
                                "supervisor",
                                Priority::Hard,
                                BusPayload::Diagnostic {
                                    level: DiagLevel::Critical,
                                    component: name.into(),
                                    message: "task panicked, restarting".into(),
                                },
                            ));
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod restart_tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn spawn_restartable_restarts_after_panic() {
        let mut sup = Supervisor::new();
        let count = std::sync::Arc::new(AtomicU32::new(0));
        let c = count.clone();
        sup.spawn_restartable("test-panic", move || {
            let c = c.clone();
            async move {
                let n = c.fetch_add(1, Ordering::SeqCst);
                if n < 2 { panic!("intentional panic #{}", n); }
                // на третьем запуске - выходим
            }
        });
        let token = sup.token();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
            token.cancel();
        });
        sup.run().await;
        assert!(count.load(Ordering::SeqCst) >= 3, "должно быть >=3 запусков, got {}", count.load(Ordering::SeqCst));
    }
}