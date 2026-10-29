//! src/monitoring/mod.rs - per-core CPU collection.
pub mod ebpf_cpu;
pub mod fallback;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("ebpf unavailable: {0}")]
    EbpfUnavailable(String),
}

pub trait CpuSource: Send + Sync {
    fn read(&self) -> Vec<f64>;
    fn name(&self) -> &'static str;
}

pub fn create_cpu_source(kind: &str) -> Box<dyn CpuSource> {
    match kind {
        "ebpf" => match ebpf_cpu::EbpfCpu::new() {
            Ok(s) => Box::new(s),
            Err(e) => {
                tracing::warn!(error = %e, "ebpf unavailable, fallback to procstat");
                Box::new(fallback::ProcStatCpu::new())
            }
        },
        _ => Box::new(fallback::ProcStatCpu::new()),
    }
}

pub fn avg(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        0.0
    } else {
        slice.iter().sum::<f64>() / slice.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avg_empty() {
        assert_eq!(avg(&[]), 0.0);
    }

    #[test]
    fn avg_simple() {
        assert_eq!(avg(&[0.0, 1.0]), 0.5);
        assert!((avg(&[0.2, 0.4, 0.6]) - 0.4).abs() < 1e-9, "float avg");
    }

    #[test]
    fn create_fallback() {
        let s = create_cpu_source("procstat");
        assert_eq!(s.name(), "procstat");
    }
}
