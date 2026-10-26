//! src/hard_pool/scheduler.rs - periodic scheduler.
//!
//! Not recursive submit. Single separate thread stores list of periodic tasks
//! and re-submits them to pool for each period.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::hard_pool::{HardPool, PoolError};

pub(crate) struct PeriodicEntry {
    pub period: Duration,
    pub func: Arc<dyn Fn() + Send + Sync>,
    pub next: Instant,
}

static ENTRIES: Mutex<Vec<PeriodicEntry>> = Mutex::new(Vec::new());
static STARTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn register_periodic(
    pool: Arc<HardPool>,
    period: Duration,
    func: Arc<dyn Fn() + Send + Sync>,
) -> Result<(), PoolError> {
    if period.is_zero() {
        return Err(PoolError::Full(0));
    }
    {
        let mut g = ENTRIES.lock().unwrap();
        g.push(PeriodicEntry {
            period,
            func,
            next: Instant::now() + period,
        });
    }
    ensure_timer(pool);
    Ok(())
}

fn ensure_timer(pool: Arc<HardPool>) {
    if STARTED.swap(true, std::sync::atomic::Ordering::AcqRel) {
        return;
    }
    std::thread::Builder::new()
        .name("hard-sched".into())
        .spawn(move || timer_loop(pool))
        .expect("spawn hard-sched");
}

fn timer_loop(pool: Arc<HardPool>) {
    let tick = Duration::from_millis(1);
    loop {
        if pool.is_stopped() {
            break;
        }
        let now = Instant::now();
        let due: Vec<Arc<dyn Fn() + Send + Sync>> = {
            let mut g = ENTRIES.lock().unwrap();
            let mut due = Vec::new();
            for e in g.iter_mut() {
                if e.next <= now {
                    due.push(Arc::clone(&e.func));
                    e.next = now + e.period;
                }
            }
            due
        };
        for f in due {
            let f2 = Arc::clone(&f);
            let _ = pool.submit(Duration::from_millis(10), move || f2());
        }
        std::thread::sleep(tick);
    }
}
