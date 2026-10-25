//! src/gateway/tls.rs - rustls + mTLS.
//!
//! Builds ServerConfig with mandatory client certificate (if require_mtls).
//! Checks that certificate files exist and parse.

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::{RootCertStore, ServerConfig};

use super::{GatewayConfig, GatewayError};

pub fn build_server_config(cfg: &GatewayConfig) -> Result<Arc<ServerConfig>, GatewayError> {
    cfg.validate()?;

    let ca_certs = load_certs(&cfg.tls.ca)?;
    let mut roots = RootCertStore::empty();
    for c in ca_certs {
        roots
            .add(c)
            .map_err(|e| GatewayError::Tls(format!("ca add: {e}")))?;
    }

    let verifier = if cfg.tls.client_cert_required {
        WebPkiClientVerifier::builder(Arc::new(roots))
            .build()
            .map_err(|e| GatewayError::Tls(format!("client verifier: {e}")))?
    } else {
        return build_without_client_auth(cfg);
    };

    let certs = load_certs(&cfg.tls.cert)?;
    let key = load_key(&cfg.tls.key)?;

    let mut server_cfg = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(certs, key)
        .map_err(|e| GatewayError::Tls(format!("server config: {e}")))?;

    server_cfg.alpn_protocols = vec![b"http/1.1".to_vec(), b"h2".to_vec()];

    Ok(Arc::new(server_cfg))
}

fn build_without_client_auth(cfg: &GatewayConfig) -> Result<Arc<ServerConfig>, GatewayError> {
    let certs = load_certs(&cfg.tls.cert)?;
    let key = load_key(&cfg.tls.key)?;
    let server_cfg = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| GatewayError::Tls(format!("server config: {e}")))?;
    Ok(Arc::new(server_cfg))
}

fn load_certs(path: &std::path::Path) -> Result<Vec<CertificateDer<'static>>, GatewayError> {
    let f = File::open(path)
        .map_err(|e| GatewayError::Tls(format!("open cert {}: {e}", path.display())))?;
    let mut rd = BufReader::new(f);
    let certs: Result<Vec<_>, _> = rustls_pemfile::certs(&mut rd).collect();
    let certs = certs.map_err(|e| GatewayError::Tls(format!("parse cert {}: {e}", path.display())))?;
    if certs.is_empty() {
        return Err(GatewayError::Tls(format!("no certs in {}", path.display())));
    }
    Ok(certs)
}

fn load_key(path: &std::path::Path) -> Result<PrivateKeyDer<'static>, GatewayError> {
    let f = File::open(path)
        .map_err(|e| GatewayError::Tls(format!("open key {}: {e}", path.display())))?;
    let mut rd = BufReader::new(f);
    let key = rustls_pemfile::private_key(&mut rd)
        .map_err(|e| GatewayError::Tls(format!("parse key {}: {e}", path.display())))?;
    key.ok_or_else(|| GatewayError::Tls(format!("no key in {}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::{AuditConfig, ServerConfig, TerminalConfig, TlsConfig};
    use std::path::PathBuf;

    fn cfg_with_mtls() -> GatewayConfig {
        GatewayConfig {
            server: ServerConfig {
                dashboard_addr: "0.0.0.0:8443".into(),
                terminal_addr: "0.0.0.0:8444".into(),
                require_mtls: true,
                min_tls_version: "1.3".into(),
            },
            tls: TlsConfig {
                cert: PathBuf::from("certs/server.crt"),
                key: PathBuf::from("certs/server.key"),
                ca: PathBuf::from("certs/ca.crt"),
                client_cert_required: true,
            },
            terminal: TerminalConfig {
                allowed_domains: vec!["core".into()],
                allowed_commands: vec!["diag".into()],
                deny_shell: true,
                audit_file: PathBuf::from("logs/terminal-audit.log"),
                session_timeout_sec: 1800,
                max_concurrent: 4,
            },
            audit: AuditConfig {
                enabled: true,
                file: PathBuf::from("logs/terminal-audit.log"),
                to_diagnostics: true,
            },
        }
    }

    #[test]
    fn validate_rejects_mismatched_mtls() {
        let mut c = cfg_with_mtls();
        c.tls.client_cert_required = false;
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_bad_tls_version() {
        let mut c = cfg_with_mtls();
        c.server.min_tls_version = "1.1".into();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_allowlist_with_deny_shell() {
        let mut c = cfg_with_mtls();
        c.terminal.allowed_commands.clear();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_audit_mismatch() {
        let mut c = cfg_with_mtls();
        c.audit.file = PathBuf::from("other.log");
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_config() {
        assert!(cfg_with_mtls().validate().is_ok());
    }

    #[test]
    fn build_server_config_fails_on_missing_files() {
        let c = cfg_with_mtls();
        let err = build_server_config(&c).unwrap_err();
        assert!(matches!(err, GatewayError::Tls(_)));
    }
}
