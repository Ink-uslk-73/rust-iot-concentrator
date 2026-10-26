//! src/hard_pool/worker.rs - pool worker thread.
//!
//! Each worker: 1 OS thread, 1 core, SCHED_FIFO 90 (if possible).
//! If SCHED_FIFO unavailable (no CAP_SYS_NICE) - runs as SCHED_OTHER,
//! but writes warning to diagnostics.

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;

use crate::core::bus::{BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::hard_pool::HardPool;

pub(crate) fn run_worker(pool: Arc<HardPool>, idx: usize, core: usize) {
    apply_rt_settings(idx, core, pool.bus());

    loop {
        if pool.is_stopped() {
            break;
        }
        match pool.try_pop() {
            Some(task) => {
                if Instant::now() > task.deadline {
                    pool.stats.expired.fetch_add(1, Ordering::Relaxed);
                    let _ = pool.bus().diagnostics.try_send(BusMessage::new(
                        "hard_pool",
                        Priority::Hard,
                        BusPayload::Diagnostic {
                            level: DiagLevel::Warning,
                            component: "hard_pool".into(),
                            message: format!("task {} expired before exec", task.id),
                        },
                    ));
                    continue;
                }

                let t0 = Instant::now();
                (task.func)();
                let dt = t0.elapsed();

                pool.stats.executed.fetch_add(1, Ordering::Relaxed);
                if dt > std::time::Duration::from_millis(10) {
                    pool.stats.sla_breaches.fetch_add(1, Ordering::Relaxed);
                    let _ = pool.bus().diagnostics.try_send(BusMessage::new(
                        "hard_pool",
                        Priority::Hard,
                        BusPayload::Diagnostic {
                            level: DiagLevel::Warning,
                            component: "hard_pool".into(),
                            message: format!("task {} SLA breach: {} us", task.id, dt.as_micros()),
                        },
                    ));
                }
            }
            None => {
                std::hint::spin_loop();
                if idx % 64 == 0 {
                    std::thread::yield_now();
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn apply_rt_settings(idx: usize, core: usize, bus: &crate::core::bus::Bus) {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut set);
        libc::CPU_SET(core, &mut set);
        let _ = libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set);

        let param = libc::sched_param { sched_priority: 90 };
        let rc = libc::sched_setscheduler(0, libc::SCHED_FIFO, &param);
        if rc != 0 {
            let e = std::io::Error::last_os_error();
            let _ = bus.diagnostics.try_send(BusMessage::new(
                "hard_pool",
                Priority::Hard,
                BusPayload::Diagnostic {
                    level: DiagLevel::Warning,
                    component: "hard_pool".into(),
                    message: format!(
                        "worker {}: SCHED_FIFO failed ({}), fallback SCHED_OTHER",
                        idx, e
                    ),
                },
            ));
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn apply_rt_settings(_idx: usize, _core: usize, _bus: &crate::core::bus::Bus) {}
