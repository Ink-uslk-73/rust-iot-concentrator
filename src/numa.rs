//! src/numa.rs - Привязка потоков к NUMA-узлам и ядрам.
//!
//! Для 2x AMD EPYC 7002 (2 NUMA node, 64 ядра/сокет):
//! - HARD -> node 0, cores 0-7, SCHED_FIFO 90, mlockall
//! - SOFT -> node 0, cores 8-31, SCHED_OTHER
//! - CORE -> node 0, cores 32-47, SCHED_OTHER
//! - LIFE -> node 1, cores 64-95, SCHED_OTHER
//! - STORAGE -> node 1, cores 96-127, SCHED_OTHER
//!
//! Per-thread affinity + set_mempolicy. НЕ numactl --membind=0 на весь процесс.

use std::collections::HashSet;
use serde::Deserialize;
use thiserror::Error;
#[allow(unused_imports)]
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum NumaError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("config invalid: {0}")]
    Config(String),
    #[error("syscall failed: {0} (errno {1})")]
    Syscall(&'static str, i32),
}

#[derive(Debug, Deserialize)]
pub struct NumaConfig {
    pub topology: Topology,
    pub domain: std::collections::HashMap<String, DomainNuma>,
    #[serde(default)]
    pub validation: Validation,
}

#[derive(Debug, Deserialize)]
pub struct Topology {
    pub sockets: u32,
    pub cores_per_socket: u32,
    pub numa_nodes: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DomainNuma {
    pub numa_node: u32,
    pub cores: Vec<usize>,
    pub scheduler: String,
    #[serde(default)]
    pub priority: i32,
    pub mlockall: bool,
    #[serde(default)]
    pub isolated: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Validation {
    #[serde(default)]
    pub forbid_overlap: bool,
    #[serde(default)]
    pub require_isolated_disjoint: bool,
}

#[derive(Debug, Clone)]
pub struct HostTopology {
    pub online_cores: usize,
    pub numa_nodes: Vec<Vec<usize>>,
}

impl HostTopology {
    pub fn read() -> Self {
        let online = std::fs::read_to_string("/sys/devices/system/cpu/online")
            .ok()
            .map(|s| parse_cpu_range(s.trim()))
            .unwrap_or_else(|| (0..1).collect());

        let mut numa_nodes: Vec<Vec<usize>> = vec![Vec::new()];
        let base = std::path::Path::new("/sys/devices/system/node");
        if let Ok(rd) = std::fs::read_dir(base) {
            let mut entries: Vec<_> = rd.flatten().collect();
            entries.sort_by_key(|e| e.file_name());
            let mut nodes = Vec::new();
            for e in entries {
                let name = e.file_name();
                let name = name.to_string_lossy();
                if let Some(rest) = name.strip_prefix("node") {
                    if let Ok(id) = rest.parse::<usize>() {
                        let cpulist = std::fs::read_to_string(e.path().join("cpulist"))
                            .ok()
                            .unwrap_or_default();
                        let cores = parse_cpu_range(cpulist.trim());
                        if !cores.is_empty() {
                            nodes.push((id, cores));
                        }
                    }
                }
            }
            if !nodes.is_empty() {
                let max_id = nodes.iter().map(|(id, _)| *id).max().unwrap_or(0);
                numa_nodes = vec![Vec::new(); max_id + 1];
                for (id, cores) in nodes {
                    numa_nodes[id] = cores;
                }
            }
        }

        Self {
            online_cores: online.len(),
            numa_nodes,
        }
    }

    pub fn numa_node_of(&self, core: usize) -> Option<usize> {
        self.numa_nodes.iter().position(|cores| cores.contains(&core))
    }
}

pub fn parse_cpu_range(s: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((a, b)) = part.split_once('-') {
            if let (Ok(a), Ok(b)) = (a.trim().parse::<usize>(), b.trim().parse::<usize>()) {
                out.extend(a..=b);
            }
        } else if let Ok(v) = part.parse::<usize>() {
            out.push(v);
        }
    }
    out
}

impl NumaConfig {
    pub fn validate(&self) -> Result<(), NumaError> {
        if self.topology.sockets == 0 || self.topology.cores_per_socket == 0 {
            return Err(NumaError::Config("topology: sockets/cores_per_socket must be > 0".into()));
        }
        let max_cores = (self.topology.sockets * self.topology.cores_per_socket) as usize;

        for (name, d) in &self.domain {
            if d.numa_node >= self.topology.numa_nodes {
                return Err(NumaError::Config(format!(
                    "domain {}: numa_node {} >= {}",
                    name, d.numa_node, self.topology.numa_nodes
                )));
            }
            for &c in &d.cores {
                if c >= max_cores {
                    return Err(NumaError::Config(format!(
                        "domain {}: core {} >= {}",
                        name, c, max_cores
                    )));
                }
            }
            let sched_ok = matches!(d.scheduler.as_str(), "fifo" | "rr" | "other");
            if !sched_ok {
                return Err(NumaError::Config(format!(
                    "domain {}: scheduler '{}' not in [fifo, rr, other]",
                    name, d.scheduler
                )));
            }
        }

        if self.validation.forbid_overlap {
            let mut seen: HashSet<usize> = HashSet::new();
            for (name, d) in &self.domain {
                for &c in &d.cores {
                    if !seen.insert(c) {
                        return Err(NumaError::Config(format!(
                            "core {} used in multiple domains (at {})",
                            c, name
                        )));
                    }
                }
            }
        }

        if self.validation.require_isolated_disjoint {
            let isolated: HashSet<usize> = self
                .domain
                .values()
                .filter(|d| d.isolated)
                .flat_map(|d| d.cores.iter().copied())
                .collect();
            for (name, d) in &self.domain {
                if d.isolated {
                    continue;
                }
                for &c in &d.cores {
                    if isolated.contains(&c) {
                        return Err(NumaError::Config(format!(
                            "core {} isolated and also in domain {}",
                            c, name
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, NumaError> {
        let s = std::fs::read_to_string(path)?;
        let cfg: NumaConfig = toml::from_str(&s)?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn domain(&self, name: &str) -> Option<&DomainNuma> {
        self.domain.get(name)
    }
}

#[cfg(target_os = "linux")]
pub fn apply_domain(domain: &DomainNuma, domain_name: &str) -> Result<(), NumaError> {
    if domain.cores.is_empty() {
        return Err(NumaError::Config(format!("domain {}: empty cores", domain_name)));
    }
    set_cpu_affinity(&domain.cores)?;
    set_mempolicy_preferred(domain.numa_node)?;
    match domain.scheduler.as_str() {
        "fifo" => set_scheduler_fifo(domain.priority.max(1))?,
        "rr" => set_scheduler_rr(domain.priority.max(1))?,
        _ => {}
    }
    if domain.mlockall {
        mlock_all()?;
    }
    info!(
        domain = domain_name,
        numa_node = domain.numa_node,
        cores = domain.cores.len(),
        scheduler = domain.scheduler,
        mlockall = domain.mlockall,
        "domain bound"
    );
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn apply_domain(_domain: &DomainNuma, _domain_name: &str) -> Result<(), NumaError> {
    warn!("apply_domain: no-op (not Linux)");
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_cpu_affinity(cores: &[usize]) -> Result<(), NumaError> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut set);
        for &c in cores {
            if c < libc::CPU_SETSIZE as usize {
                libc::CPU_SET(c, &mut set);
            } else {
                return Err(NumaError::Config(format!(
                    "core {} >= CPU_SETSIZE {}",
                    c, libc::CPU_SETSIZE
                )));
            }
        }
        let ret = libc::sched_setaffinity(
            0,
            std::mem::size_of::<libc::cpu_set_t>(),
            &set,
        );
        if ret != 0 {
            return Err(NumaError::Syscall("sched_setaffinity", errno()));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_mempolicy_preferred(node: u32) -> Result<(), NumaError> {
    const MPOL_PREFERRED: libc::c_int = 1;
    unsafe {
        let mut mask: libc::c_ulong = 0;
        if node < (std::mem::size_of::<libc::c_ulong>() * 8) as u32 {
            mask = 1u64 << node;
        } else {
            return Err(NumaError::Config(format!("node {} too large", node)));
        }
        let ret = libc::syscall(
            libc::SYS_set_mempolicy,
            MPOL_PREFERRED,
            &mask as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::c_ulong>() * 8,
        );
        if ret != 0 {
            return Err(NumaError::Syscall("set_mempolicy", errno()));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_scheduler_fifo(priority: i32) -> Result<(), NumaError> {
    unsafe {
        let param = libc::sched_param { sched_priority: priority };
        let ret = libc::sched_setscheduler(0, libc::SCHED_FIFO, &param);
        if ret != 0 {
            return Err(NumaError::Syscall("sched_setscheduler(FIFO)", errno()));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_scheduler_rr(priority: i32) -> Result<(), NumaError> {
    unsafe {
        let param = libc::sched_param { sched_priority: priority };
        let ret = libc::sched_setscheduler(0, libc::SCHED_RR, &param);
        if ret != 0 {
            return Err(NumaError::Syscall("sched_setscheduler(RR)", errno()));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn mlock_all() -> Result<(), NumaError> {
    unsafe {
        let ret = libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
        if ret != 0 {
            return Err(NumaError::Syscall("mlockall", errno()));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn valid_cfg() -> NumaConfig {
        let mut domain = HashMap::new();
        domain.insert(
            "hard".to_string(),
            DomainNuma {
                numa_node: 0,
                cores: vec![0, 1, 2, 3, 4, 5, 6, 7],
                scheduler: "fifo".into(),
                priority: 90,
                mlockall: true,
                isolated: true,
            },
        );
        domain.insert(
            "life".to_string(),
            DomainNuma {
                numa_node: 1,
                cores: vec![64, 65, 66],
                scheduler: "other".into(),
                priority: 0,
                mlockall: false,
                isolated: false,
            },
        );
        NumaConfig {
            topology: Topology { sockets: 2, cores_per_socket: 64, numa_nodes: 2 },
            domain,
            validation: Validation {
                forbid_overlap: true,
                require_isolated_disjoint: true,
            },
        }
    }

    #[test]
    fn parse_cpu_range_simple() {
        assert_eq!(parse_cpu_range("0-3"), vec![0, 1, 2, 3]);
        assert_eq!(parse_cpu_range("5"), vec![5]);
        assert_eq!(parse_cpu_range("0-1,4,6-7"), vec![0, 1, 4, 6, 7]);
        assert_eq!(parse_cpu_range(""), Vec::<usize>::new());
    }

    #[test]
    fn validate_ok() {
        assert!(valid_cfg().validate().is_ok());
    }

    #[test]
    fn validate_rejects_bad_node() {
        let mut cfg = valid_cfg();
        cfg.domain.get_mut("hard").unwrap().numa_node = 5;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_rejects_core_out_of_range() {
        let mut cfg = valid_cfg();
        cfg.domain.get_mut("hard").unwrap().cores = vec![0, 200];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_rejects_overlap() {
        let mut cfg = valid_cfg();
        cfg.domain.get_mut("life").unwrap().cores.push(3);
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_rejects_bad_scheduler() {
        let mut cfg = valid_cfg();
        cfg.domain.get_mut("hard").unwrap().scheduler = "deadline".into();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn host_topology_read_does_not_panic() {
        let t = HostTopology::read();
        assert!(t.online_cores >= 1);
        assert!(!t.numa_nodes.is_empty());
    }

    #[test]
    fn host_topology_numa_node_of() {
        let t = HostTopology {
            online_cores: 4,
            numa_nodes: vec![vec![0, 1], vec![2, 3]],
        };
        assert_eq!(t.numa_node_of(0), Some(0));
        assert_eq!(t.numa_node_of(3), Some(1));
        assert_eq!(t.numa_node_of(99), None);
    }
}
