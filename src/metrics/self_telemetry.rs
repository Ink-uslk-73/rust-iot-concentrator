//! src/metrics/self_telemetry.rs - self-diagnostics (CPU, mem, disks, NUMA, temps).
//!
//! sysinfo 0.32 API: global_cpu_usage(), Disks::new_with_refreshed_list().

use sysinfo::{Disks, System};

use crate::metrics::registry::Registry;

pub struct SelfTelemetry {
    sys: System,
    disks: Disks,
    pub numa_nodes: usize,
}

impl SelfTelemetry {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(250));
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let numa_nodes = if cfg!(target_os = "linux") { 2 } else { 0 };
        Self { sys, disks, numa_nodes }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh_list();
    }

    pub fn publish(&mut self, registry: &Registry) {
        self.refresh();

        // CPU per-core
        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            registry.set_gauge(format!("self.cpu.core.{}", i), cpu.cpu_usage() as f64 / 100.0);
        }
        // global_cpu_usage() returns f32 (0..100)
        let avg = self.sys.global_cpu_usage() as f64 / 100.0;
        registry.set_gauge("self.cpu.avg", avg);

        // Memory
        let total = self.sys.total_memory() as f64;
        let used = self.sys.used_memory() as f64;
        registry.set_gauge("self.mem.total", total);
        registry.set_gauge("self.mem.used", used);
        registry.set_gauge("self.mem.used_ratio", if total > 0.0 { used / total } else { 0.0 });

        // Disks (sysinfo 0.32: Disks, not System::disks())
        for disk in self.disks.list() {
            let n = disk.name().to_string_lossy();
            registry.set_gauge(format!("self.disk.{}.total", n), disk.total_space() as f64);
            registry.set_gauge(format!("self.disk.{}.free", n), disk.available_space() as f64);
        }

        // NUMA: Linux -> real (placeholder; P1.6 numastat), Windows -> zeros
        if self.numa_nodes == 0 {
            registry.set_gauge("self.numa.node.0.cores", 0.0);
            registry.set_gauge("self.numa.node.1.cores", 0.0);
            registry.set_gauge("self.numa.node.0.mem_used_ratio", 0.0);
            registry.set_gauge("self.numa.node.1.mem_used_ratio", 0.0);
            registry.set_gauge("self.numa.foreign", 0.0);
        } else {
            registry.set_gauge("self.numa.node.0.cores", 8.0);
            registry.set_gauge("self.numa.node.1.cores", 8.0);
            registry.set_gauge("self.numa.foreign", 0.0);
        }

        // Temperatures: Linux only
        #[cfg(target_os = "linux")]
        {
            for (i, comp) in self.sys.components().iter().enumerate() {
                registry.set_gauge(format!("self.temp.{}", i), comp.temperature() as f64);
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            registry.set_gauge("self.temp.cpu", 0.0);
        }
    }
}

impl Default for SelfTelemetry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn publish_writes_cpu_avg() {
        let mut st = SelfTelemetry::new();
        let r = Arc::new(Registry::new());
        st.publish(&r);
        assert!(r.get_gauge("self.cpu.avg").is_some());
    }

    #[test]
    fn numa_zeros_on_windows() {
        let st = SelfTelemetry::new();
        if cfg!(not(target_os = "linux")) {
            assert_eq!(st.numa_nodes, 0);
        }
    }
}
