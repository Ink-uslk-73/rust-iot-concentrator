//! src/terminal/allowlist.rs - command and domain checks.

use crate::gateway::TerminalConfig;

pub struct Allowlist {
    domains: Vec<String>,
    commands: Vec<String>,
    deny_shell: bool,
}

impl Allowlist {
    pub fn from_config(cfg: &TerminalConfig) -> Self {
        Self {
            domains: cfg.allowed_domains.clone(),
            commands: cfg.allowed_commands.clone(),
            deny_shell: cfg.deny_shell,
        }
    }

    pub fn allows_domain(&self, domain: &str) -> bool {
        self.domains.iter().any(|d| d == domain)
    }

    pub fn allows_command(&self, cmd: &str) -> bool {
        if !self.deny_shell {
            return true;
        }
        let prog = cmd.split_whitespace().next().unwrap_or("");
        self.commands.iter().any(|c| c == prog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn cfg(deny: bool) -> TerminalConfig {
        TerminalConfig {
            allowed_domains: vec!["core".into(), "vm-hard".into()],
            allowed_commands: vec!["diag".into(), "logs".into()],
            deny_shell: deny,
            audit_file: PathBuf::from("logs/audit.log"),
            session_timeout_sec: 1800,
            max_concurrent: 4,
        }
    }

    #[test]
    fn allows_known_domain() {
        let a = Allowlist::from_config(&cfg(true));
        assert!(a.allows_domain("core"));
        assert!(!a.allows_domain("vm-soft"));
    }

    #[test]
    fn deny_shell_restricts() {
        let a = Allowlist::from_config(&cfg(true));
        assert!(a.allows_command("diag"));
        assert!(!a.allows_command("rm -rf /"));
    }

    #[test]
    fn no_deny_allows_all() {
        let a = Allowlist::from_config(&cfg(false));
        assert!(a.allows_command("anything"));
    }
}
