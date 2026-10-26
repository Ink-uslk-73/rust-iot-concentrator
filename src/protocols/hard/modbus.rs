//! src/protocols/hard/modbus.rs - Modbus RTU poller for AIG-502.
//!
//! Runs on AIG-502 (Debian 11). Opens /dev/ttyM0, /dev/ttyM1 directly.
//! Config: ThingsPro config.json + our registers.json.
//! Loop: read config -> poll slaves -> publish telemetry -> repeat.

use std::time::Duration;
use tokio::time;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

use crate::core::mode_switch::Priority;
use crate::protocols::{ProtocolAdapter, PublishFn};
use super::modbus_config::ModbusConfig;

pub struct ModbusAdapter {
    config_path: String,
    registers_path: String,
    refresh_every: u32,
}

impl ModbusAdapter {
    pub fn new(config_path: impl Into<String>, registers_path: impl Into<String>) -> Self {
        Self {
            config_path: config_path.into(),
            registers_path: registers_path.into(),
            refresh_every: 10,
        }
    }
}

impl ProtocolAdapter for ModbusAdapter {
    fn name(&self) -> &'static str { "modbus" }
    fn priority(&self) -> Priority { Priority::Hard }

    fn start(&self, publish: PublishFn) -> Result<(), String> {
        let cfg_path = self.config_path.clone();
        let reg_path = self.registers_path.clone();
        let refresh = self.refresh_every;

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .expect("modbus runtime");

            rt.block_on(async move {
                let mut config: Option<ModbusConfig> = None;
                let mut cycle: u32 = 0;

                loop {
                    if config.is_none() || cycle % refresh == 0 {
                        match ModbusConfig::load(&cfg_path, &reg_path) {
                            Ok(c) => {
                                tracing::info!(devices = c.devices.len(), "modbus config");
                                config = Some(c);
                            }
                            Err(e) => tracing::warn!("config: {}", e),
                        }
                    }

                    let Some(cfg) = config.as_ref() else {
                        time::sleep(Duration::from_millis(1000)).await;
                        cycle = cycle.wrapping_add(1);
                        continue;
                    };

                    for dev in &cfg.devices {
                        let builder = tokio_serial::new(&dev.port, dev.baud)
                            .data_bits(match dev.data_bits {
                                7 => tokio_serial::DataBits::Seven,
                                _ => tokio_serial::DataBits::Eight,
                            })
                            .stop_bits(match dev.stop_bits {
                                2 => tokio_serial::StopBits::Two,
                                _ => tokio_serial::StopBits::One,
                            })
                            .parity(match dev.parity.as_str() {
                                "even" => tokio_serial::Parity::Even,
                                "odd" => tokio_serial::Parity::Odd,
                                _ => tokio_serial::Parity::None,
                            })
                            .flow_control(tokio_serial::FlowControl::None);

                        let stream = match SerialStream::open(&builder) {
                            Ok(s) => s,
                            Err(e) => {
                                tracing::warn!(port = %dev.port, "serial: {}", e);
                                continue;
                            }
                        };

                        let mut ctx = rtu::attach_slave(stream, Slave(dev.slave));
                       match ctx.read_holding_registers(dev.start_addr, dev.quantity).await {
    Ok(data) => {
        for (i, &val) in data.iter().enumerate() {
            publish(Priority::Hard, val as f64, "reg");
            tracing::debug!(slave = dev.slave, reg = dev.start_addr + i as u16, value = val, "modbus");
        }
    }
    Err(e) => tracing::warn!(slave = dev.slave, "read: {}", e),
}
                    }

                    time::sleep(Duration::from_millis(cfg.poll_ms)).await;
                    cycle = cycle.wrapping_add(1);
                }
            });
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_constructs() {
        let a = ModbusAdapter::new("/var/thingspro/data/config.json", "/var/thingspro/data/registers.json");
        assert_eq!(a.name(), "modbus");
        assert_eq!(a.priority(), Priority::Hard);
    }
}
