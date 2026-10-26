//! src/monitoring/ebpf_cpu.rs - eBPF per-core CPU (stub until aya integration).
//!
//! Full aya + BPF program integration is a separate step (requires .bpf.c
//! and cargo-xtask). Here - a frame with explicit error, so that
//! create_cpu_source("ebpf") falls back to procstat honestly.
//!
//! When aya is connected (feature "ebpf"):
//! - BPF program tracks finish_task_switch
//! - PERCPU_ARRAY 128 slots
//! - read via map -> Vec<f64>

use super::{CpuSource, MonitorError};

pub struct EbpfCpu;

impl EbpfCpu {
    pub fn new() -> Result<Self, MonitorError> {
        #[cfg(feature = "ebpf")]
        {
            return Err(MonitorError::EbpfUnavailable(
                "aya feature enabled but .bpf.c not built yet".into(),
            ));
        }
        #[cfg(not(feature = "ebpf"))]
        {
            Err(MonitorError::EbpfUnavailable(
                "feature 'ebpf' not enabled at compile time".into(),
            ))
        }
    }
}

impl CpuSource for EbpfCpu {
    fn read(&self) -> Vec<f64> {
        Vec::new()
    }
    fn name(&self) -> &'static str {
        "ebpf"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_gracefully() {
        let r = EbpfCpu::new();
        assert!(r.is_err());
    }
}
