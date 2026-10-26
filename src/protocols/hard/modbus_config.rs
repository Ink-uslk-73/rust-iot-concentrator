//! src/protocols/hard/modbus_config.rs - config from AIG-502.
//!
//! Reads two files:
//! - /var/thingspro/data/config.json  -> ThingsPro serials
//! - /var/thingspro/data/registers.json -> register map + poll_ms
//!
//! Validation:
//! - port_id must exist in config.json
//! - quantity <= 125 (Modbus function 3 limit)
//! - poll_ms >= 1

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
    #[serde(default = "default_poll_ms")]
    pub poll_ms: u64,
}

fn default_poll_ms() -> u64 { 50 }

/// A resolved device: port settings + one register range.
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

/// Group of devices sharing the same physical port.
#[derive(Debug, Clone)]
pub struct PortGroup {
    pub port: String,
    pub baud: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
    pub devices: Vec<ResolvedDevice>,
}

#[derive(Debug, Clone)]
pub struct ModbusConfig {
    pub ports: Vec<PortGroup>,
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

    pub fn merge(tp: ThingsProConfig, regs: RegistersConfig) -> Result<Self, String> {
        // 1. Validate poll_ms.
        if regs.poll_ms < 1 {
            return Err(format!("poll_ms must be >= 1, got {}", regs.poll_ms));
        }

        // 2. Build ports map.
        let mut ports_map: HashMap<String, ThingsProSerial> = HashMap::new();
        for (key, val) in &tp.serials {
            if key == "arraySize" { continue; }
            if let Ok(serial) = serde_json::from_value::<ThingsProSerial>(val.clone()) {
                ports_map.insert(key.clone(), serial);
            }
        }

        // 3. Resolve devices and group by port.
        let mut groups: HashMap<String, PortGroup> = HashMap::new();
        for dev in &regs.devices {
            let Some(serial) = ports_map.get(&dev.port_id) else {
                return Err(format!("port_id {} not in config.json", dev.port_id));
            };
            for reg in &dev.registers {
                if reg.quantity == 0 {
                    return Err(format!(
                        "port_id {} slave {}: quantity must be > 0",
                        dev.port_id, reg.slave
                    ));
                }
                if reg.quantity > 125 {
                    return Err(format!(
                        "port_id {} slave {}: quantity {} > 125 (Modbus limit)",
                        dev.port_id, reg.slave, reg.quantity
                    ));
                }

                let rd = ResolvedDevice {
                    port: serial.device.clone(),
                    baud: serial.baud_rate,
                    data_bits: serial.data_bits,
                    parity: serial.parity.clone(),
                    stop_bits: serial.stop_bits,
                    slave: reg.slave,
                    start_addr: reg.start_addr,
                    quantity: reg.quantity,
                };

                groups
                    .entry(serial.device.clone())
                    .or_insert_with(|| PortGroup {
                        port: serial.device.clone(),
                        baud: serial.baud_rate,
                        data_bits: serial.data_bits,
                        parity: serial.parity.clone(),
                        stop_bits: serial.stop_bits,
                        devices: Vec::new(),
                    })
                    .devices
                    .push(rd);
            }
        }

        let mut ports: Vec<PortGroup> = groups.into_values().collect();
        ports.sort_by(|a, b| a.port.cmp(&b.port));

        Ok(ModbusConfig { ports, poll_ms: regs.poll_ms })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tp_json() -> &'static str {
        r#"{"serials":{"0":{"baudRate":9600,"dataBits":8,"device":"/dev/ttyM0","parity":"none","stopBits":1,"id":1},"1":{"baudRate":19200,"dataBits":8,"device":"/dev/ttyM1","parity":"none","stopBits":1,"id":2},"arraySize":2}}"#
    }

    #[test]
    fn merge_groups_by_port() {
        let regs_json = r#"{"devices":[
            {"port_id":"0","registers":[{"slave":1,"start_addr":0,"quantity":10},{"slave":2,"start_addr":100,"quantity":5}]},
            {"port_id":"1","registers":[{"slave":3,"start_addr":0,"quantity":20}]}
        ],"poll_ms":50}"#;
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json).unwrap();
        let cfg = ModbusConfig::merge(tp, regs).unwrap();
        assert_eq!(cfg.ports.len(), 2);
        assert_eq!(cfg.poll_ms, 50);
        let p0 = cfg.ports.iter().find(|p| p.port == "/dev/ttyM0").unwrap();
        assert_eq!(p0.devices.len(), 2);
        let p1 = cfg.ports.iter().find(|p| p.port == "/dev/ttyM1").unwrap();
        assert_eq!(p1.devices.len(), 1);
    }

    #[test]
    fn default_poll_ms_is_50() {
        let regs_json = r#"{"devices":[]}"#;
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json).unwrap();
        let cfg = ModbusConfig::merge(tp, regs).unwrap();
        assert_eq!(cfg.poll_ms, 50);
    }

    #[test]
    fn rejects_quantity_over_125() {
        let regs_json = r#"{"devices":[{"port_id":"0","registers":[{"slave":1,"start_addr":0,"quantity":200}]}],"poll_ms":50}"#;
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json).unwrap();
        assert!(ModbusConfig::merge(tp, regs).is_err());
    }

    #[test]
    fn rejects_zero_quantity() {
        let regs_json = r#"{"devices":[{"port_id":"0","registers":[{"slave":1,"start_addr":0,"quantity":0}]}],"poll_ms":50}"#;
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json).unwrap();
        assert!(ModbusConfig::merge(tp, regs).is_err());
    }

    #[test]
    fn rejects_unknown_port_id() {
        let regs_json = r#"{"devices":[{"port_id":"99","registers":[{"slave":1,"start_addr":0,"quantity":10}]}],"poll_ms":50}"#;
        let tp: ThingsProConfig = serde_json::from_str(tp_json()).unwrap();
        let regs: RegistersConfig = serde_json::from_str(regs_json).unwrap();
        assert!(ModbusConfig::merge(tp, regs).is_err());
    }
}
