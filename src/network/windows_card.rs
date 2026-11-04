//! src/network/windows_card.rs - WindowsCard (netsh/PowerShell, P3.47).
#![cfg(target_os = "windows")]

use std::process::Command;
use super::card::{NetworkCard, LinkStats};

#[derive(Debug)]
pub struct WindowsCard {
    pub name: String,
    pub mac: String,
    pub up: bool,
    pub mtu: u16,
}

fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd).args(args).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

impl NetworkCard for WindowsCard {
    fn mac_address(&self) -> &str { &self.mac }
    fn speed_mbps(&self) -> u32 { 0 }
    fn is_up(&self) -> bool { self.up }
    fn mtu(&self) -> u16 { self.mtu }
    fn up(&self) -> Result<(), String> {
        run("netsh", &["interface", "set", "interface", &self.name, "admin=enable"]).map(|_| ())
    }
    fn down(&self) -> Result<(), String> {
        run("netsh", &["interface", "set", "interface", &self.name, "admin=disable"]).map(|_| ())
    }
    fn set_mtu(&self, mtu: u16) -> Result<(), String> {
        let m = format!("mtu={}", mtu);
        run("netsh", &["interface", "ipv4", "set", "subinterface", &self.name, &m, "store=persistent"]).map(|_| ())
    }
    fn stats(&self) -> Result<LinkStats, String> {
        let script = format!("(Get-NetAdapterStatistics -Name '{}').ReceivedBytes", self.name);
        let rx = run("powershell", &["-NoProfile", "-Command", &script])?;
        let rx_bytes = rx.trim().parse().unwrap_or(0);
        Ok(LinkStats { rx_bytes, tx_bytes: 0, ..Default::default() })
    }
}
