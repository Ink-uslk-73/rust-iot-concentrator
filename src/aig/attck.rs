//! src/aig/attck.rs - map AIG events to ATT&CK for ICS.
//!
//! Minimal set (10 techniques) for first iteration. Extendable to 30.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    ConfigDrift,
    UnauthorizedSlave,
    HttpsDisabled,
    FirmwareChanged,
    NewAccount,
    PasswordChanged,
    DeviceReboot,
    AigOffline,
    BruteForce,
    DefaultCreds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttckTag {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
}

pub fn map(kind: &EventKind) -> AttckTag {
    match kind {
        EventKind::ConfigDrift => AttckTag {
            technique_id: "T0836".into(),
            technique_name: "Modify Parameter".into(),
            tactic: "Impair Process Control".into(),
        },
        EventKind::UnauthorizedSlave => AttckTag {
            technique_id: "T0855".into(),
            technique_name: "Unauthorized Command Message".into(),
            tactic: "Impair Process Control".into(),
        },
        EventKind::HttpsDisabled => AttckTag {
            technique_id: "T0836".into(),
            technique_name: "Modify Parameter".into(),
            tactic: "Impair Process Control".into(),
        },
        EventKind::FirmwareChanged => AttckTag {
            technique_id: "T0889".into(),
            technique_name: "Modify Program".into(),
            tactic: "Persistence".into(),
        },
        EventKind::NewAccount => AttckTag {
            technique_id: "T0890".into(),
            technique_name: "Create Account".into(),
            tactic: "Persistence".into(),
        },
        EventKind::PasswordChanged => AttckTag {
            technique_id: "T0891".into(),
            technique_name: "Change Credential".into(),
            tactic: "Persistence".into(),
        },
        EventKind::DeviceReboot => AttckTag {
            technique_id: "T0816".into(),
            technique_name: "Device Restart/Shutdown".into(),
            tactic: "Inhibit Response Function".into(),
        },
        EventKind::AigOffline => AttckTag {
            technique_id: "T0826".into(),
            technique_name: "Loss of Availability".into(),
            tactic: "Impact".into(),
        },
        EventKind::BruteForce => AttckTag {
            technique_id: "T0806".into(),
            technique_name: "Brute Force I/O".into(),
            tactic: "Initial Access".into(),
        },
        EventKind::DefaultCreds => AttckTag {
            technique_id: "T0812".into(),
            technique_name: "Default Credentials".into(),
            tactic: "Initial Access".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_10_techniques() {
        for k in [
            EventKind::ConfigDrift, EventKind::UnauthorizedSlave,
            EventKind::HttpsDisabled, EventKind::FirmwareChanged,
            EventKind::NewAccount, EventKind::PasswordChanged,
            EventKind::DeviceReboot, EventKind::AigOffline,
            EventKind::BruteForce, EventKind::DefaultCreds,
        ] {
            let t = map(&k);
            assert!(t.technique_id.starts_with('T'));
            assert!(!t.technique_name.is_empty());
        }
    }

    #[test]
    fn config_drift_is_t0836() {
        assert_eq!(map(&EventKind::ConfigDrift).technique_id, "T0836");
    }
}
