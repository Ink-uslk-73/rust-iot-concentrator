//! src/supervisor.rs - Supervision tree (JoinSet, CancellationToken, let-it-crash).

use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::vm::state::{VmSemaphore, VmState};

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Счётчик паник на задачу. Живёт в Supervisor, читается через Supervisor::panics().
#[derive(Debug, Default)]
pub struct PanicCounter(AtomicU64);

impl PanicCounter {
    pub fn inc(&self) -> u64 { self.0.fetch_add(1, Ordering::Relaxed) + 1 }
    pub fn get(&self) -> u64 { self.0.load(Ordering::Relaxed) }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorState { Running, Restarting, Stopped }

pub struct Supervisor {
    token: CancellationToken,
    set: JoinSet<()>,
    semaphores: Vec<(String, VmSemaphore)>,
    bus: Option<Bus>,
    panics: std::collections::HashMap<&'static str, Arc<PanicCounter>>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            set: JoinSet::new(),
            semaphores: Vec::new(),
            bus: None,
            panics: std::collections::HashMap::new(),
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

    /// Число паник по имени задачи (0, если задача не restartable).
    pub fn panics(&self, name: &str) -> u64 {
        self.panics.get(name).map(|c| c.get()).unwrap_or(0)
    }

    /// Счётчик паник для задачи; создаётся при первом обращении.
    pub fn panic_counter(&mut self, name: &'static str) -> Arc<PanicCounter> {
        self.panics.entry(name).or_insert_with(|| Arc::new(PanicCounter::default())).clone()
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
        use crate::transport::reconnect::ReconnectSchedule;

        let factory = std::sync::Arc::new(factory);
        let bus = self.bus.clone();
        let token = self.token.clone();
        let counter = self.panic_counter(name);

        self.set.spawn(async move {
            let mut sched = ReconnectSchedule::default_with_seed(crate::core::bus::next_seq());
            loop {
                if token.is_cancelled() { break; }
                let fut = factory();
                let result = std::panic::AssertUnwindSafe(fut).catch_unwind().await;
                match result {
                    Ok(()) => break,
                    Err(payload) => {
                        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = payload.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "unknown panic".into()
                        };
                        let n = counter.inc();
                        warn!(task = name, panics = n, panic_msg = %msg, "supervisor: task panicked, restart");
                        if let Some(bus) = &bus {
                            let _ = bus.diagnostics.try_send(BusMessage::new(
                                "supervisor",
                                Priority::Hard,
                                BusPayload::Diagnostic {
                                    level: DiagLevel::Critical,
                                    component: name.into(),
                                    message: format!("task panicked (n={}), restarting", n),
                                },
                            ));
                        }
                        let delay = sched.next_delay();
                        tokio::time::sleep(delay).await;
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

    #[test]
    fn panic_counter_basic() {
        let c = PanicCounter::default();
        assert_eq!(c.get(), 0);
        assert_eq!(c.inc(), 1);
        assert_eq!(c.inc(), 2);
        assert_eq!(c.get(), 2);
    }

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

    #[tokio::test]
    async fn restart_emits_bus_diagnostics_with_counter() {
        let m = std::sync::Arc::new(crate::metrics::OverheadMetrics::new());
        let (bus, mut rx) = Bus::new(4, 8, 4, 8, m);
        let mut sup = Supervisor::new().with_bus(bus);

        let c = std::sync::Arc::new(AtomicU32::new(0));
        let cc = c.clone();
        sup.spawn_restartable("diag", move || {
            let cc = cc.clone();
            async move {
                let n = cc.fetch_add(1, Ordering::SeqCst);
                if n < 2 { panic!("boom #{}", n); }
            }
        });

        let reader = tokio::spawn(async move {
            let mut seen = 0u32;
            while let Some(msg) = rx.diagnostics.recv().await {
                if let BusPayload::Diagnostic { message, .. } = &msg.payload {
                    if message.contains("task panicked (n=") { seen += 1; }
                }
            }
            seen
        });

        let token = sup.token();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
            token.cancel();
        });

        sup.run().await;
        let seen = reader.await.unwrap();
        assert!(seen >= 2, "ожидали >=2 диагностик с n=, got {}", seen);
    }
}