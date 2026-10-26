//! src/metrics/collector_cpu.rs - CPU per-core -> registry.

use std::sync::Arc;

use crate::metrics::registry::Registry;
use crate::metrics::MetricsError;
use crate::monitoring::CpuSource;

pub struct CpuCollector {
    source: Box<dyn CpuSource>,
    registry: Arc<Registry>,
}

impl CpuCollector {
    pub fn new(source: Box<dyn CpuSource>, registry: Arc<Registry>) -> Self {
        Self { source, registry }
    }

    pub fn collect(&self) -> Result<(), MetricsError> {
        let v = self.source.read();
        if v.is_empty() {
            return Err(MetricsError::Unavailable("cpu source empty".into()));
        }
        for (i, &u) in v.iter().enumerate() {
            self.registry.set_gauge(format!("cpu.core.{}", i), u);
        }
        let avg = crate::monitoring::avg(&v);
        self.registry.set_gauge("cpu.avg", avg);
        Ok(())
    }

    pub fn source_name(&self) -> &'static str {
        self.source.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockCpu { v: Mutex<Vec<f64>> }
    impl MockCpu {
        fn new(v: Vec<f64>) -> Self { Self { v: Mutex::new(v) } }
    }
    impl CpuSource for MockCpu {
        fn read(&self) -> Vec<f64> { self.v.lock().unwrap().clone() }
        fn name(&self) -> &'static str { "mock" }
    }

    #[test]
    fn collects_per_core() {
        let r = Arc::new(Registry::new());
        let c = CpuCollector::new(Box::new(MockCpu::new(vec![0.1, 0.5, 0.9])), Arc::clone(&r));
        c.collect().unwrap();
        assert_eq!(r.get_gauge("cpu.core.0"), Some(0.1));
        assert!((r.get_gauge("cpu.avg").unwrap() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn empty_source_errors() {
        let r = Arc::new(Registry::new());
        let c = CpuCollector::new(Box::new(MockCpu::new(vec![])), r);
        assert!(c.collect().is_err());
    }
}
