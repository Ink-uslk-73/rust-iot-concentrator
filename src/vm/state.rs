//! src/vm/state.rs - VmSemaphore (STOPPED/WAITING/RUNNING).

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Stopped = 0,
    Waiting = 1,
    Running = 2,
}

impl VmState {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => VmState::Stopped,
            1 => VmState::Waiting,
            _ => VmState::Running,
        }
    }
}

#[derive(Clone)]
pub struct VmSemaphore {
    state: Arc<AtomicU8>,
    notify: Arc<Notify>,
}

impl VmSemaphore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(AtomicU8::new(VmState::Stopped as u8)),
            notify: Arc::new(Notify::new()),
        }
    }

    pub fn get(&self) -> VmState {
        VmState::from_u8(self.state.load(Ordering::Acquire))
    }

    pub fn set(&self, s: VmState) {
        self.state.store(s as u8, Ordering::Release);
        self.notify.notify_waiters();
    }

    pub fn transition(&self, from: VmState, to: VmState) -> bool {
        self.state
            .compare_exchange(from as u8, to as u8, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub async fn wait_running(&self) {
        loop {
            if self.get() == VmState::Running {
                return;
            }
            self.notify.notified().await;
        }
    }
}

impl Default for VmSemaphore {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn transitions() {
        let s = VmSemaphore::new();
        assert_eq!(s.get(), VmState::Stopped);
        assert!(s.transition(VmState::Stopped, VmState::Waiting));
        assert!(s.transition(VmState::Waiting, VmState::Running));
        assert_eq!(s.get(), VmState::Running);
    }

    #[tokio::test]
    async fn wait_running_wakes() {
        let s = VmSemaphore::new();
        let s2 = s.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            s2.set(VmState::Running);
        });
        s.wait_running().await;
        assert_eq!(s.get(), VmState::Running);
    }
}
