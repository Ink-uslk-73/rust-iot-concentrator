//! src/supervisor.rs - Supervision tree (JoinSet, CancellationToken, let-it-crash).

use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::vm::state::{VmSemaphore, VmState};

pub struct Supervisor {
    token: CancellationToken,
    set: JoinSet<()>,
    semaphores: Vec<(String, VmSemaphore)>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            set: JoinSet::new(),
            semaphores: Vec::new(),
        }
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

    /// let-it-crash: wait all, restart on panic.
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
                        // let-it-crash: restart (placeholder)
                    }
                }
                else => break,
            }
        }
        // structured: join all
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
