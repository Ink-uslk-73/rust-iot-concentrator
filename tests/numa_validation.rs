use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize)] struct NumaCfg { topology: Topology, domain: HashMap<String, DomainNuma>, validation: Validation }
#[derive(Deserialize)] struct Topology { sockets: u32, cores_per_socket: u32, numa_nodes: u32 }
#[derive(Deserialize)] struct DomainNuma { numa_node: u32, cores: Vec<usize>, scheduler: String, mlockall: bool, #[serde(default)] isolated: bool }
#[derive(Deserialize)] struct Validation { forbid_overlap: bool, require_isolated_disjoint: bool }

fn load() -> NumaCfg { let s = std::fs::read_to_string("config/numa.toml").expect("read"); toml::from_str(&s).expect("parse") }

#[test] fn topology() { let c = load(); assert_eq!(c.topology.sockets, 2); assert_eq!(c.topology.cores_per_socket, 64); }
#[test] fn all_domains() { let c = load(); for d in ["hard","soft","core","life","storage"] { assert!(c.domain.contains_key(d)); } }
#[test] fn no_overlap() { let c = load(); if !c.validation.forbid_overlap { return; } let mut seen = HashSet::new(); for (n, d) in &c.domain { for &x in &d.cores { assert!(seen.insert(x), "core {} dup in {}", x, n); } } }
#[test] fn within_topology() { let c = load(); let max = (c.topology.sockets * c.topology.cores_per_socket) as usize; for (n, d) in &c.domain { for &x in &d.cores { assert!(x < max, "core {} >= {} in {}", x, max, n); } } }
#[test] fn node_range() { let c = load(); for (n, d) in &c.domain { assert!(d.numa_node < c.topology.numa_nodes, "{}", n); } }
#[test] fn hard_rt() { let c = load(); let h = &c.domain["hard"]; assert_eq!(h.numa_node, 0); assert_eq!(h.scheduler, "fifo"); assert!(h.mlockall); assert!(h.isolated); assert_eq!(h.cores.len(), 8); }
#[test] fn life_node1() { assert_eq!(load().domain["life"].numa_node, 1); }
#[test] fn storage_node1() { assert_eq!(load().domain["storage"].numa_node, 1); }
