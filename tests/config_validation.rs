use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)] struct ConcentratorCfg { general: General, domain: HashMap<String, DomainCfg>, mode: ModeCfg, balancer: BalancerCfg, watchdog: WatchdogCfg, bus: BusCfg, gateway: GatewayCfg, metrics: MetricsCfg }
#[derive(Deserialize)] struct General { name: String, version: String }
#[derive(Deserialize)] struct DomainCfg { enabled: bool }
#[derive(Deserialize)] struct ModeCfg { default: String, manual_switch: bool }
#[derive(Deserialize)] struct BalancerCfg { high_threshold: f64, critical_threshold: f64, recover_threshold: f64, hysteresis_ticks: u32, per_node: bool }
#[derive(Deserialize)] struct WatchdogCfg { hard_timeout_ms: u64, soft_timeout_ms: u64, life_timeout_ms: u64, storage_timeout_ms: u64 }
#[derive(Deserialize)] struct BusCfg { signals_capacity: usize, telemetry_capacity: usize }
#[derive(Deserialize)] struct GatewayCfg { dashboard_addr: String, terminal_addr: String, require_mtls: bool }
#[derive(Deserialize)] struct MetricsCfg { cpu_source: String }

fn load() -> ConcentratorCfg {
    let s = std::fs::read_to_string("config/concentrator.toml").expect("read");
    toml::from_str(&s).expect("parse")
}

#[test] fn parses() { let c = load(); assert_eq!(c.general.name, "concentrator-01"); }
#[test] fn has_domains() { let c = load(); for d in ["hard","soft","life","storage"] { assert!(c.domain.contains_key(d)); } }
#[test] fn mode_valid() { let c = load(); assert!(["hard_only","soft_only","life_only","hard_soft","hard_life","soft_life","all"].contains(&c.mode.default.as_str())); }
#[test] fn balancer_ordered() { let c = load(); assert!(c.balancer.recover_threshold < c.balancer.high_threshold); assert!(c.balancer.high_threshold < c.balancer.critical_threshold); }
#[test] fn watchdog_ordered() { let c = load(); assert!(c.watchdog.hard_timeout_ms < c.watchdog.soft_timeout_ms); assert!(c.watchdog.soft_timeout_ms < c.watchdog.life_timeout_ms); }
#[test] fn mtls() { let c = load(); assert!(c.gateway.require_mtls); assert!(c.gateway.dashboard_addr.ends_with(":8443")); }
#[test] fn cpu_source() { let c = load(); assert!(["ebpf","procstat"].contains(&c.metrics.cpu_source.as_str())); }
