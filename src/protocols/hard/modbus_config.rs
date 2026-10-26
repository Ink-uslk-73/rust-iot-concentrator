//! src/protocols/hard/modbus_config.rs - config from AIG-502 (ThingsPro JSON + registers).
//!
//! Two files:
//! - /var/thingspro/data/config.json  -> ThingsPro serials (device, baudRate, parity, ...)
//! - /var/thingspro/data/registers.json -> our register map (port_id -> slaves/registers)

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ThingsProSerial {
    #[serde(rename = "baudRate")]
    pub baud_rate: u32,
    #[serde(rename = "dataBits")]
    pub data_bits: u8,
    pub device: String,
    pub parity: String,
    #[serde(rename = "stopBits")]
    pub stop_bits: u8,
    pub id: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThingsProConfig {
    pub serials: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterMap {
    pub slave: u8,
    pub start_addr: u16,
    pub quantity: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceRegisters {
    pub port_id: String,
    pub registers: Vec<RegisterMap>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistersConfig {
    pub devices: Vec<DeviceRegisters>,
}

#[derive(Debug, Clone)]
pub struct ResolvedDevice {
    pub port: String,
    pub baud: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
    pub slave: u8,
    pub start_addr: u16,
    pub quantity: u16,
}

#[derive(Debug, Clone)]
pub struct ModbusConfig {
    pub devices: Vec<ResolvedDevice>,
    pub poll_ms: u64,
}

impl ModbusConfig {
    pub fn load(config_path: &str, registers_path: &str) -> Result<Self, String> {
        let tp = Self::load_thingspro(config_path)?;
        let regs = Self::load_registers(registers_path)?;
        Self::merge(tp, regs)
    }

    fn load_thingspro(path: &str) -> Result<ThingsProConfig, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("read {}: {}", path, e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("parse {}: {}", path, e))
    }

    fn load_registers(path: &str) -> Result<RegistersConfig, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("read {}: {}", path, e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("parse {}: {}", path, e))
    }

    fn merge(tp: ThingsProConfig, regs: RegistersConfig) -> Result<Self, String> {
        let mut ports: HashMap<String, ThingsProSerial> = HashMap::new();
        for (key, val) in &tp.serials {
            if key == "arraySize" {
                continue;
            }
            if let Ok(serial) = serde_json::from_value::<ThingsProSerial>(val.clone()) {
                ports.insert(key.clone(), serial);
            }
        }

        let mut devices = Vec::new();
        for dev in &regs.devices {
            let Some(serial) = ports.get(&dev.port_id) else {
                return Err(format!("port_id {} not in config.json", dev.port_id));
            };
            for reg in &dev.registers {
                devices.push(ResolvedDevice {
                    port: serial.device.clone(),
                    baud: serial.baud_rate,
                    data_bits: serial.data_bits,
                    parity: serial.parity.clone(),
                    stop_bits: serial.stop_bits,
                    slave: reg.slave,
                    start_addr: reg.start_addr,
                    quantity: reg.quantity,
                });
            }
        }
        Ok(ModbusConfig { devices, poll_ms: 10 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn tp_json() -> &'static str {
        r#"{"serials":{"0":{"baudRate":9600,"dataBits":8,"device":"/dev/ttyM0","parity":"none","stopBits":1,"id":1},"arraySize":1}}"#
    }
    fn regs_json() -> &'static str {
        r#"{"devices":[{"port_id":"0","registers":[{"slave":1,"start_addr":0,"quantity":10}]}]}"#
    }

    #[test]
    fn merge_ok() {
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json()).unwrap();
        let cfg = ModbusConfig::merge(tp, regs).unwrap();
        assert_eq!(cfg.devices.len(), 1);
        assert_eq!(cfg.devices[0].port, "/dev/ttyM0");
        assert_eq!(cfg.devices[0].baud, 9600);
    }

    #[test]
    fn load_from_files() {
        let dir = tempfile::tempdir().unwrap();
        let tp = dir.path().join("config.json");
        let rg = dir.path().join("registers.json");
        std::fs::File::create(&tp).unwrap().write_all(tp_json().as_bytes()).unwrap();
        std::fs::File::create(&rg).unwrap().write_all(regs_json().as_bytes()).unwrap();
        let cfg = ModbusConfig::load(tp.to_str().unwrap(), rg.to_str().unwrap()).unwrap();
        assert_eq!(cfg.devices.len(), 1);
    }
}
