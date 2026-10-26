//! src/hard_pool/mod.rs - HARD thread pool (RT, bounded, backpressure).
//!
//! Fixes vs v3.x:
//! - bounded queue (not unbounded SegQueue) -> backpressure, no drop.
//! - periodic scheduler (scheduler.rs), not recursive submit.
//! - worker = 1 OS thread = 1 core, SCHED_FIFO 90, affinity from numa.
//! - tasks with deadline; expired -> diagnostics, not executed.

pub mod scheduler;
pub mod worker;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam::queue::ArrayQueue;
use thiserror::Error;
use tracing::info;

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("queue full (capacity {0})")]
    Full(usize),
    #[error("pool stopped")]
    Stopped,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub struct HardTask {
    pub id: u64,
    pub deadline: Instant,
    pub func: Box<dyn FnOnce() + Send + 'static>,
}

impl std::fmt::Debug for HardTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HardTask")
            .field("id", &self.id)
            .field("deadline", &self.deadline)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct PoolStats {
    pub submitted: AtomicU64,
    pub executed: AtomicU64,
    pub dropped_full: AtomicU64,
    pub expired: AtomicU64,
    pub sla_breaches: AtomicU64,
}

impl PoolStats {
    pub fn snapshot(&self) -> PoolStatsSnapshot {
        PoolStatsSnapshot {
            submitted: self.submitted.load(Ordering::Relaxed),
            executed: self.executed.load(Ordering::Relaxed),
            dropped_full: self.dropped_full.load(Ordering::Relaxed),
            expired: self.expired.load(Ordering::Relaxed),
            sla_breaches: self.sla_breaches.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStatsSnapshot {
    pub submitted: u64,
    pub executed: u64,
    pub dropped_full: u64,
    pub expired: u64,
    pub sla_breaches: u64,
}

pub struct HardPool {
    queue: Arc<ArrayQueue<HardTask>>,
    stop: Arc<AtomicBool>,
    next_id: AtomicU64,
    pub stats: Arc<PoolStats>,
    bus: Bus,
    capacity: usize,
}

impl HardPool {
    pub fn start(
        threads: usize,
        cores: std::ops::Range<usize>,
        capacity: usize,
        bus: Bus,
    ) -> Result<Arc<Self>, PoolError> {
        if threads == 0 {
            return Err(PoolError::Full(0));
        }
        let cap = capacity.max(threads * 8);
        let pool = Arc::new(Self {
            queue: Arc::new(ArrayQueue::new(cap)),
            stop: Arc::new(AtomicBool::new(false)),
            next_id: AtomicU64::new(1),
            stats: Arc::new(PoolStats::default()),
            bus,
            capacity: cap,
        });

        for i in 0..threads {
            let core = cores.start + (i % cores.len().max(1));
            let p = Arc::clone(&pool);
            std::thread::Builder::new()
                .name(format!("hard-{}", i))
                .spawn(move || worker::run_worker(p, i, core))?;
        }

        info!(threads, capacity = cap, cores = ?cores, "hard pool started");
        Ok(pool)
    }

    pub fn submit(&self, deadline: Duration, func: impl FnOnce() + Send + 'static) -> Result<u64, PoolError> {
        if self.stop.load(Ordering::Acquire) {
            return Err(PoolError::Stopped);
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let task = HardTask {
            id,
            deadline: Instant::now() + deadline,
            func: Box::new(func),
        };
        if self.queue.push(task).is_err() {
            self.stats.dropped_full.fetch_add(1, Ordering::Relaxed);
            let _ = self.bus.diagnostics.try_send(BusMessage::new(
                "hard_pool",
                Priority::Hard,
                BusPayload::Diagnostic {
                    level: DiagLevel::Critical,
                    component: "hard_pool".into(),
                    message: format!("queue full (capacity {})", self.capacity),
                },
            ));
            return Err(PoolError::Full(self.capacity));
        }
        self.stats.submitted.fetch_add(1, Ordering::Relaxed);
        Ok(id)
    }

    pub fn submit_periodic(
        self: &Arc<Self>,
        period: Duration,
        func: impl Fn() + Send + Sync + 'static,
    ) -> Result<(), PoolError> {
        scheduler::register_periodic(Arc::clone(self), period, Arc::new(func))
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }

    pub fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub(crate) fn try_pop(&self) -> Option<HardTask> {
        self.queue.pop()
    }

    pub(crate) fn bus(&self) -> &Bus {
        &self.bus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    fn make_pool(threads: usize, cap: usize) -> Arc<HardPool> {
        let (bus, _rx) = Bus::new(64, 64, 64, 64);
        HardPool::start(threads, 0..threads, cap, bus).unwrap()
    }

    #[test]
    fn pool_starts_and_stops() {
        let p = make_pool(2, 16);
        assert!(!p.is_stopped());
        p.stop();
        assert!(p.is_stopped());
    }

    #[test]
    fn submit_runs_task() {
        let p = make_pool(2, 16);
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);
        p.submit(Duration::from_millis(100), move || {
            c.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
        for _ in 0..100 {
            if counter.load(Ordering::SeqCst) == 1 {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        p.stop();
    }

    #[test]
    fn submit_after_stop_fails() {
        let p = make_pool(1, 4);
        p.stop();
        let err = p.submit(Duration::from_millis(10), || {}).unwrap_err();
        assert!(matches!(err, PoolError::Stopped));
    }

    #[test]
    fn stats_submitted_increments() {
        let p = make_pool(2, 16);
        p.submit(Duration::from_secs(10), || {}).unwrap();
        p.submit(Duration::from_secs(10), || {}).unwrap();
        let s = p.stats.snapshot();
        assert!(s.submitted >= 2);
        p.stop();
    }

    #[test]
    fn periodic_registers_and_runs() {
        let p = make_pool(2, 32);
        let count = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&count);
        p.submit_periodic(Duration::from_millis(10), move || {
            c.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
        std::thread::sleep(Duration::from_millis(80));
        p.stop();
        assert!(count.load(Ordering::SeqCst) >= 2);
    }
}
