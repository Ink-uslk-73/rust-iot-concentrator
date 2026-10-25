use serde::Deserialize;

#[derive(Deserialize)] struct GatewayFile { server: Server, tls: Tls, terminal: Terminal, audit: Audit }
#[derive(Deserialize)] struct Server { dashboard_addr: String, terminal_addr: String, require_mtls: bool, min_tls_version: String }
#[derive(Deserialize)] struct Tls { cert: String, key: String, ca: String, client_cert_required: bool }
#[derive(Deserialize)] struct Terminal { allowed_domains: Vec<String>, allowed_commands: Vec<String>, deny_shell: bool, audit_file: String }
#[derive(Deserialize)] struct Audit { enabled: bool, file: String, to_diagnostics: bool }

fn load() -> GatewayFile { let s = std::fs::read_to_string("config/gateway.toml").expect("read"); toml::from_str(&s).expect("parse") }

#[test] fn mtls() { let g = load(); assert!(g.server.require_mtls); assert!(g.tls.client_cert_required); assert_eq!(g.server.min_tls_version, "1.3"); }
#[test] fn ports() { let g = load(); assert!(g.server.dashboard_addr.ends_with(":8443")); assert!(g.server.terminal_addr.ends_with(":8444")); }
#[test] fn deny_shell() { let g = load(); assert!(g.terminal.deny_shell); }
#[test] fn allowlist() { let g = load(); for c in ["diag","logs","top","df","smartctl"] { assert!(g.terminal.allowed_commands.iter().any(|x| x == c)); } }
#[test] fn audit() { let g = load(); assert!(g.audit.enabled); assert_eq!(g.terminal.audit_file, g.audit.file); }
