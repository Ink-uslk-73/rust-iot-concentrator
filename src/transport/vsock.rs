//! src/transport/vsock.rs - VSOCK (CORE <-> SOFT/LIFE/STORAGE).
//!
//! Linux only. On Windows - stub (returns error).
//! AF_VSOCK, SOCK_STREAM.

use super::{Transport, TransportMessage};

pub struct VsockTransport {
    _cid: u32,
    _port: u32,
}

impl VsockTransport {
    pub fn new(_cid: u32, _port: u32) -> Result<Self, String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("vsock not supported on this OS".into());
        }
        #[cfg(target_os = "linux")]
        {
            Ok(Self { _cid, _port })
        }
    }
}

impl Transport for VsockTransport {
    fn send(&self, _msg: TransportMessage) -> Result<(), String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("vsock not supported on this OS".into());
        }
        #[cfg(target_os = "linux")]
        {
            // TODO: AF_VSOCK socket
            Ok(())
        }
    }
    fn recv(&self) -> Option<TransportMessage> {
        None
    }
    fn name(&self) -> &'static str { "vsock" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vsock_linux_only() {
        let r = VsockTransport::new(2, 5000);
        if cfg!(target_os = "linux") {
            assert!(r.is_ok());
        } else {
            assert!(r.is_err());
        }
    }
}
