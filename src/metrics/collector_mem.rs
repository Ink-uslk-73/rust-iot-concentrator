//! src/metrics/collector_mem.rs - memory per-NUMA-node.

use std::path::Path;
use std::sync::Arc;

use crate::metrics::registry::Registry;
use crate::metrics::MetricsError;

pub struct MemCollector {
    registry: Arc<Registry>,
    base: std::path::PathBuf,
}

impl MemCollector {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self {
            registry,
            base: Path::new("/sys/devices/system/node").to_path_buf(),
        }
    }

    #[cfg(test)]
    pub fn with_base(registry: Arc<Registry>, base: impl Into<std::path::PathBuf>) -> Self {
        Self { registry, base: base.into() }
    }

    pub fn collect(&self) -> Result<(), MetricsError> {
        let entries = std::fs::read_dir(&self.base)?;
        let mut found = 0;
        for e in entries.flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            let Some(rest) = name.strip_prefix("node") else { continue };
            let Ok(node) = rest.parse::<usize>() else { continue };
            let meminfo = e.path().join("meminfo");
            let Ok(content) = std::fs::read_to_string(&meminfo) else { continue };
            let mut total = 0u64;
            let mut free = 0u64;
            for line in content.lines() {
                if let Some(idx) = line.find("MemTotal:") {
                    if let Some(v) = parse_kb(&line[idx + "MemTotal:".len()..]) {
                        total = v;
                    }
                } else if let Some(idx) = line.find("MemFree:") {
                    if let Some(v) = parse_kb(&line[idx + "MemFree:".len()..]) {
                        free = v;
                    }
                }
            }
            if total > 0 {
                found += 1;
                self.registry.set_gauge(format!("mem.node.{}.total", node), total as f64);
                self.registry.set_gauge(format!("mem.node.{}.free", node), free as f64);
                self.registry.set_gauge(
                    format!("mem.node.{}.used_ratio", node),
                    1.0 - (free as f64 / total as f64),
                );
            }
        }
        if found == 0 {
            return Err(MetricsError::Unavailable("no numa nodes".into()));
        }
        Ok(())
    }
}

fn parse_kb(s: &str) -> Option<u64> {
    let mut it = s.split_whitespace();
    it.next()?.parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_kb_simple() {
        assert_eq!(parse_kb("  65536 kB"), Some(65536));
        assert_eq!(parse_kb(""), None);
    }

    #[test]
    fn collects_from_tempdir() {
        let dir = tempfile::tempdir().unwrap();
        let node0 = dir.path().join("node0");
        std::fs::create_dir_all(&node0).unwrap();
        let mut f = std::fs::File::create(node0.join("meminfo")).unwrap();
        writeln!(f, "Node 0 MemTotal:       1000 kB").unwrap();
        writeln!(f, "Node 0 MemFree:         200 kB").unwrap();

        let r = Arc::new(Registry::new());
        let c = MemCollector::with_base(Arc::clone(&r), dir.path());
        c.collect().unwrap();
        assert_eq!(r.get_gauge("mem.node.0.total"), Some(1000.0));
        assert!((r.get_gauge("mem.node.0.used_ratio").unwrap() - 0.8).abs() < 1e-9);
    }
}
