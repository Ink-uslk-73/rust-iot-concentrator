//! src/metrics/mod.rs - metric registry + collectors + snapshot.
pub mod collector_cpu;
pub mod collector_disk;
pub mod collector_mem;
pub mod collector_net;
pub mod registry;
pub mod snapshot;
pub mod statistics;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("collector unavailable: {0}")]
    Unavailable(String),
}

pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self) -> Result<(), MetricsError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn avg_helper() {
        assert_eq!(crate::monitoring::avg(&[0.0, 1.0]), 0.5);
    }
}
