//! src/raw/wifi_source.rs - WifiSource (rfmon + pcap, Linux-only, P3.37 часть 3).
#![cfg(target_os = "linux")]

use pcap::{Capture, Device};
use rfmon::MonitorGuard;
use super::source::RawSource;

pub struct WifiSource {
    name: String,
    cap: Capture<pcap::Active>,
    _guard: MonitorGuard,
}

impl WifiSource {
    pub async fn open() -> Result<Self, String> {
        let mon = rfmon::start_monitor().await.map_err(|e| e.to_string())?;
        let name = mon.name().to_string();
        let dev = Device::list()
            .map_err(|e| e.to_string())?
            .into_iter()
            .find(|d| d.name == name)
            .ok_or_else(|| format!("monitor interface not found: {}", name))?;
        let cap = Capture::from_device(dev)
            .map_err(|e| e.to_string())?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()
            .map_err(|e| e.to_string())?;
        Ok(Self { name, cap, _guard: mon })
    }
}

impl RawSource for WifiSource {
    fn name(&self) -> &str { &self.name }
    fn next(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self.cap.next_packet() {
            Ok(pkt) => Ok(Some(pkt.data.to_vec())),
            Err(pcap::Error::TimeoutExpired) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}
