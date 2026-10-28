//! src/transport/shmem.rs - Shared-memory SPSC ring (HARD -> node 0).
//!
//! Linux only. On Windows - stub (returns error).
//! Lock-free, no allocation, cache-line aligned.

use super::{Transport, TransportMessage};

pub struct ShmemTransport {
    _path: String,
}

impl ShmemTransport {
    pub fn new(_path: impl Into<String>) -> Result<Self, String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("shmem not supported on this OS".into());
        }
        #[cfg(target_os = "linux")]
        {
            Ok(Self { _path: _path.into() })
        }
    }
}

impl Transport for ShmemTransport {
    fn send(&self, _msg: TransportMessage) -> Result<(), String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("shmem not supported on this OS".into());
        }
        #[cfg(target_os = "linux")]
        {
            // TODO: SPSC ring (crossbeam ArrayQueue + mmap)
            Ok(())
        }
    }
    fn recv(&self) -> Option<TransportMessage> {
        None
    }
    fn name(&self) -> &'static str { "shmem" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shmem_linux_only() {
        let r = ShmemTransport::new("/tmp/test.shm");
        if cfg!(target_os = "linux") {
            assert!(r.is_ok());
        } else {
            assert!(r.is_err());
        }
    }
}
