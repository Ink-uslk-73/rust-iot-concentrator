//! src/config.rs - concentrator config (TOML).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConcentratorConfig {
    pub modbus: ModbusConfig,
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
