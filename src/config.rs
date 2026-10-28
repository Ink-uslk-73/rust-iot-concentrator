//! src/config.rs - concentrator config (TOML).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConcentratorConfig {
    pub modbus: ModbusConfig,
    #[serde(default)]
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ApiConfig {
    #[serde(default)]
    pub addr: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModbusConfig {
    pub mode: String,
    pub production: ModbusPaths,
    pub emulation: ModbusPaths,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModbusPaths {
    pub config_path: String,
    pub registers_path: String,
}

impl ModbusConfig {
    pub fn current_paths(&self) -> (String, String) {
        let p = if self.mode == "emulation" { &self.emulation } else { &self.production };
        (p.config_path.clone(), p.registers_path.clone())
    }
}

impl ConcentratorConfig {
    pub fn load(path: &str) -> Result<Self, String> {
        let s = std::fs::read_to_string(path)
            .map_err(|e| format!("read {}: {}", path, e))?;
        toml::from_str(&s).map_err(|e| format!("parse {}: {}", path, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_production() {
        let cfg = ModbusConfig {
            mode: "production".into(),
            production: ModbusPaths { config_path: "/prod/c.json".into(), registers_path: "/prod/r.json".into() },
            emulation: ModbusPaths { config_path: "emu/c.json".into(), registers_path: "emu/r.json".into() },
        };
        let (c, _r) = cfg.current_paths();
        assert_eq!(c, "/prod/c.json");
    }

    #[test]
    fn api_default_none() {
        let api = ApiConfig::default();
        assert!(api.addr.is_none());
    }
}

