//! src/transport/peer_link.rs - PeerLink: local P2P / RAM-fallback.
//!
//! - `LocalP2p` — по `Platform`: Linux (заглушка над ShmemTransport, SPSC ring TBD),
//!   Windows — `RamFallback`.
//! - `RamFallback` — in-process `tokio::mpsc` (тест на разрыв/восстановление).
//!
//! Cross-socket P2P / ACS / xGMI — у концентратора, не у сателлита.

use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;

use crate::core::platform::Platform;
use crate::transport::frame::Frame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkKind { LocalP2p, RamFallback, CrossSocketP2p }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkHealth { Alive, Degraded, Down }

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("transport: {0}")] Transport(String),
    #[error("closed")] Closed,
    #[error("down")] Down,
}

#[async_trait]
pub trait PeerLink: Send + Sync {
    async fn send(&self, f: Frame) -> Result<(), LinkError>;
    async fn recv(&self) -> Result<Frame, LinkError>;
    fn kind(&self) -> LinkKind;
    fn health(&self) -> LinkHealth;
    async fn reconnect(&self) -> Result<(), LinkError>;
}

pub struct RamFallback {
    tx: mpsc::Sender<Frame>,
    alive: AtomicBool,
}

impl RamFallback {
    pub fn new(cap: usize) -> (Self, mpsc::Receiver<Frame>) {
        let (tx, rx) = mpsc::channel(cap);
        (Self { tx, alive: AtomicBool::new(true) }, rx)
    }
    pub fn set_alive(&self, alive: bool) { self.alive.store(alive, Ordering::SeqCst); }
}

#[async_trait]
impl PeerLink for RamFallback {
    async fn send(&self, f: Frame) -> Result<(), LinkError> {
        if !self.alive.load(Ordering::SeqCst) { return Err(LinkError::Down); }
        self.tx.send(f).await.map_err(|_| LinkError::Closed)
    }
    async fn recv(&self) -> Result<Frame, LinkError> { Err(LinkError::Closed) }
    fn kind(&self) -> LinkKind { LinkKind::RamFallback }
    fn health(&self) -> LinkHealth {
        if self.alive.load(Ordering::SeqCst) { LinkHealth::Alive } else { LinkHealth::Down }
    }
    async fn reconnect(&self) -> Result<(), LinkError> {
        self.alive.store(true, Ordering::SeqCst); Ok(())
    }
}

pub struct LocalP2p {
    inner: Box<dyn PeerLink>,
    platform: Platform,
}

impl LocalP2p {
    pub fn new(platform: Platform, ram_cap: usize) -> Self {
        let (ram, _rx) = RamFallback::new(ram_cap);
        let inner: Box<dyn PeerLink> = match platform {
            Platform::Linux => Box::new(ram),
            Platform::Windows => Box::new(ram),
            Platform::Other => Box::new(ram),
        };
        Self { inner, platform }
    }
    pub fn platform(&self) -> Platform { self.platform }
}

#[async_trait]
impl PeerLink for LocalP2p {
    async fn send(&self, f: Frame) -> Result<(), LinkError> { self.inner.send(f).await }
    async fn recv(&self) -> Result<Frame, LinkError> { self.inner.recv().await }
    fn kind(&self) -> LinkKind { self.inner.kind() }
    fn health(&self) -> LinkHealth { self.inner.health() }
    async fn reconnect(&self) -> Result<(), LinkError> { self.inner.reconnect().await }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::satellite::satellite_id::SatelliteId;
    fn f(seq: u64) -> Frame { Frame::with_seq(seq, SatelliteId::A, SatelliteId::C, vec![], 1000) }
    #[tokio::test]
    async fn ram_send_recv() {
        let (tx, mut rx) = RamFallback::new(4);
        tx.send(f(1)).await.unwrap();
        assert_eq!(rx.recv().await.unwrap().seq, 1);
    }
    #[tokio::test]
    async fn ram_down_after_break() {
        let (tx, _rx) = RamFallback::new(4);
        tx.set_alive(false);
        assert_eq!(tx.health(), LinkHealth::Down);
        assert!(matches!(tx.send(f(1)).await, Err(LinkError::Down)));
    }
    #[tokio::test]
    async fn ram_reconnect_heals() {
        let (tx, _rx) = RamFallback::new(4);
        tx.set_alive(false);
        tx.reconnect().await.unwrap();
        assert_eq!(tx.health(), LinkHealth::Alive);
        tx.send(f(1)).await.unwrap();
    }
    #[tokio::test]
    async fn local_p2p_uses_ram_stub() {
        let link = LocalP2p::new(Platform::Linux, 4);
        assert_eq!(link.kind(), LinkKind::RamFallback);
    }
}

