//! src/metrics/collector_disk.rs - disks (24 HDD).

use std::sync::Arc;

use crate::metrics::registry::Registry;
use crate::metrics::MetricsError;

pub struct DiskCollector {
    registry: Arc<Registry>,
    base: std::path::PathBuf,
}

impl DiskCollector {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self { registry, base: std::path::PathBuf::from("/sys/block") }
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
            let dev = name.to_string_lossy().to_string();
            if !dev.starts_with("sd") && !dev.starts_with("nvme") {
                continue;
            }
            let stat = e.path().join("stat");
            let Ok(s) = std::fs::read_to_string(&stat) else { continue };
            let fields: Vec<u64> = s
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if fields.len() < 8 {
                continue;
            }
            found += 1;
            self.registry.set_gauge(format!("disk.{}.read_ios", dev), fields[0] as f64);
            self.registry.set_gauge(format!("disk.{}.read_sectors", dev), fields[2] as f64);
            self.registry.set_gauge(format!("disk.{}.write_ios", dev), fields[4] as f64);
            self.registry.set_gauge(format!("disk.{}.write_sectors", dev), fields[6] as f64);
        }
        if found == 0 {
            return Err(MetricsError::Unavailable("no disks in sysfs".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn collects_sd_and_nvme() {
        let dir = tempfile::tempdir().unwrap();
        let sda = dir.path().join("sda");
        let loop0 = dir.path().join("loop0");
        std::fs::create_dir_all(&sda).unwrap();
        std::fs::create_dir_all(&loop0).unwrap();

        let mut f = std::fs::File::create(sda.join("stat")).unwrap();
        writeln!(f, "100 0 200 0 300 0 400 0 0 0 0").unwrap();

        let r = Arc::new(Registry::new());
        let c = DiskCollector::with_base(Arc::clone(&r), dir.path());
        c.collect().unwrap();
        assert_eq!(r.get_gauge("disk.sda.read_ios"), Some(100.0));
        assert_eq!(r.get_gauge("disk.loop0.read_ios"), None);
    }
}
