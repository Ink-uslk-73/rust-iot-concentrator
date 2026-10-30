//! src/metrics/overhead.rs - overhead metrics for concentrator.
//!
//! Measures added latency, jitter, buffer utilization, drops.
//! HARD: < 1 ms. API: < 1 ms. SSE: jitter < 10 ms. Bus buffer: < 70%.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverheadStatus {
    Ok,
    Degraded,
    Critical,
}

#[derive(Debug, Clone, Copy)]
pub struct OverheadSnapshot {
    pub hard_latency_us: u64,
    pub api_latency_us: u64,
    pub sse_jitter_ms: u64,
    pub bus_buffer_pct: u64,
    pub bus_drops: u64,
    pub sse_retransmits: u64,
    pub status: OverheadStatus,
}

pub struct OverheadMetrics {
    pub hard_latency_us: AtomicU64,
    pub api_latency_us: AtomicU64,
    pub sse_jitter_ms: AtomicU64,
    pub bus_buffer_pct: AtomicU64,
    pub bus_drops: AtomicU64,
    pub sse_retransmits: AtomicU64,
}

impl OverheadMetrics {
    pub fn new() -> Self {
        Self {
            hard_latency_us: AtomicU64::new(0),
            api_latency_us: AtomicU64::new(0),
            sse_jitter_ms: AtomicU64::new(0),
            bus_buffer_pct: AtomicU64::new(0),
            bus_drops: AtomicU64::new(0),
            sse_retransmits: AtomicU64::new(0),
        }
    }

    pub fn record_hard_latency(&self, t: Instant) {
        let us = t.elapsed().as_micros() as u64;
        self.hard_latency_us.fetch_max(us, Ordering::Relaxed);
    }

    pub fn record_api_latency(&self, t: Instant) {
        let us = t.elapsed().as_micros() as u64;
        self.api_latency_us.fetch_max(us, Ordering::Relaxed);
    }

    pub fn record_sse_jitter(&self, jitter_ms: u64) {
        self.sse_jitter_ms.fetch_max(jitter_ms, Ordering::Relaxed);
    }

    pub fn record_bus_buffer(&self, capacity: usize, free: usize) {
        let pct = if capacity == 0 { 0 } else {
            ((capacity.saturating_sub(free)) as f64 / capacity as f64 * 100.0) as u64
        };
        self.bus_buffer_pct.fetch_max(pct.min(100), Ordering::Relaxed);
    }

    pub fn record_bus_drop(&self) {
        self.bus_drops.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_sse_retransmit(&self) {
        self.sse_retransmits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> OverheadSnapshot {
        let hard = self.hard_latency_us.load(Ordering::Relaxed);
        let api = self.api_latency_us.load(Ordering::Relaxed);
        let jitter = self.sse_jitter_ms.load(Ordering::Relaxed);
        let buf = self.bus_buffer_pct.load(Ordering::Relaxed);
        let drops = self.bus_drops.swap(0, Ordering::Relaxed);
        let retr = self.sse_retransmits.load(Ordering::Relaxed);
        let status = if drops > 0 || buf > 90 || hard > 5000 {
            OverheadStatus::Critical
        } else if hard > 1000 || api > 1000 || jitter > 10 || buf > 70 {
            OverheadStatus::Degraded
        } else {
            OverheadStatus::Ok
        };
        OverheadSnapshot {
            hard_latency_us: hard,
            api_latency_us: api,
            sse_jitter_ms: jitter,
            bus_buffer_pct: buf,
            bus_drops: drops,
            sse_retransmits: retr,
            status,
        }
    }
}

impl Default for OverheadMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn record_hard_latency() {
        let m = OverheadMetrics::new();
        let t = Instant::now();
        std::thread::sleep(Duration::from_millis(2));
        m.record_hard_latency(t);
        let s = m.snapshot();
        assert!(s.hard_latency_us >= 2000);
    }

    #[test]
    fn bus_buffer_pct() {
        let m = OverheadMetrics::new();
        m.record_bus_buffer(100, 30);
        assert_eq!(m.snapshot().bus_buffer_pct, 70);
    }

    #[test]
    fn status_ok() {
        let m = OverheadMetrics::new();
        assert_eq!(m.snapshot().status, OverheadStatus::Ok);
    }

    #[test]
    fn status_degraded_on_high_latency() {
        let m = OverheadMetrics::new();
        m.hard_latency_us.store(1500, Ordering::Relaxed);
        assert_eq!(m.snapshot().status, OverheadStatus::Degraded);
    }

    #[test]
    fn status_critical_on_drops() {
        let m = OverheadMetrics::new();
        m.record_bus_drop();
        assert_eq!(m.snapshot().status, OverheadStatus::Critical);
    }
}
