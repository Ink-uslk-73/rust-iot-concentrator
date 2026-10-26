//! src/monitoring/fallback.rs - /proc/stat (per-core) if eBPF unavailable.
//!
//! Reads cpu0..cpuN from /proc/stat, computes delta idle/total between calls.
//! First call returns 0.0 for all cores.

use std::sync::Mutex;

use super::CpuSource;

pub struct ProcStatCpu {
    prev: Mutex<Vec<(u64, u64)>>,
}

impl ProcStatCpu {
    pub fn new() -> Self {
        Self { prev: Mutex::new(Vec::new()) }
    }

    fn read_raw(&self) -> Vec<(u64, u64)> {
        let mut out = Vec::new();
        let content = match std::fs::read_to_string("/proc/stat") {
            Ok(c) => c,
            Err(_) => return out,
        };
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("cpu") {
                let rest = rest.trim_start();
                if rest.starts_with(|c: char| c.is_ascii_digit()) {
                    let fields: Vec<u64> = rest
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    if fields.len() >= 4 {
                        let idle = fields[3] + fields.get(4).copied().unwrap_or(0);
                        let total: u64 = fields.iter().take(8).sum();
                        out.push((total, idle));
                    }
                }
            }
        }
        out
    }
}

impl Default for ProcStatCpu {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuSource for ProcStatCpu {
    fn read(&self) -> Vec<f64> {
        let cur = self.read_raw();
        let mut prev = self.prev.lock().unwrap();
        let mut usage = Vec::with_capacity(cur.len());

        if prev.len() != cur.len() {
            *prev = cur;
            return vec![0.0; prev.len()];
        }

        for (i, (t, id)) in cur.iter().enumerate() {
            let (pt, pid) = prev[i];
            let dt = t.saturating_sub(pt);
            let di = id.saturating_sub(pid);
            let u = if dt == 0 {
                0.0
            } else {
                1.0 - (di as f64 / dt as f64)
            };
            usage.push(u.clamp(0.0, 1.0));
        }
        *prev = cur;
        usage
    }

    fn name(&self) -> &'static str {
        "procstat"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_call_returns_zeros() {
        let s = ProcStatCpu::new();
        let v = s.read();
        #[cfg(target_os = "linux")]
        assert!(!v.is_empty());
        #[cfg(not(target_os = "linux"))]
        assert!(v.is_empty());
        assert!(v.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn second_call_in_range() {
        let s = ProcStatCpu::new();
        let _ = s.read();
        std::thread::sleep(std::time::Duration::from_millis(20));
        let v = s.read();
        for &u in &v {
            assert!((0.0..=1.0).contains(&u));
        }
    }
}
