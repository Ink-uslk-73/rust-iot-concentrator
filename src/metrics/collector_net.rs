//! src/metrics/collector_net.rs - network per-interface.

use std::sync::Arc;

use crate::metrics::registry::Registry;
use crate::metrics::MetricsError;

pub struct NetCollector {
    registry: Arc<Registry>,
    base: std::path::PathBuf,
}

impl NetCollector {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self { registry, base: std::path::PathBuf::from("/sys/class/net") }
    }

    #[cfg(test)]
    pub fn with_base(registry: Arc<Registry>, base: impl Into<std::path::PathBuf>) -> Self {
        Self { registry, base: base.into() }
    }

    pub fn collect(&self) -> Result<(), MetricsError> {
        let entries = std::fs::read_dir(&self.base)?;
        let mut found = 0;
        for e in entries.flatten() {
            let iface = e.file_name().to_string_lossy().to_string();
            if iface == "lo" {
                continue;
            }
            let stats = e.path().join("statistics");
            if !stats.exists() {
                continue;
            }
            found += 1;
            for metric in ["rx_bytes", "tx_bytes", "rx_packets", "tx_packets", "rx_errors", "tx_errors"] {
                if let Ok(s) = std::fs::read_to_string(stats.join(metric)) {
                    if let Ok(v) = s.trim().parse::<u64>() {
                        self.registry
                            .set_gauge(format!("net.{}.{}", iface, metric), v as f64);
                    }
                }
            }
        }
        if found == 0 {
            return Err(MetricsError::Unavailable("no network interfaces".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn collects_interfaces() {
        let dir = tempfile::tempdir().unwrap();
        let eth0 = dir.path().join("eth0/statistics");
        std::fs::create_dir_all(&eth0).unwrap();
        let mut f = std::fs::File::create(eth0.join("rx_bytes")).unwrap();
        write!(f, "12345\n").unwrap();

        let r = Arc::new(Registry::new());
        let c = NetCollector::with_base(Arc::clone(&r), dir.path());
        c.collect().unwrap();
        assert_eq!(r.get_gauge("net.eth0.rx_bytes"), Some(12345.0));
    }
}
