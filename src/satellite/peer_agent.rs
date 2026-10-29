//! src/satellite/peer_agent.rs - агент сателлита: PeerLink + Bus + StoreForward + PanicCounter.
//!
//! Живёт на сателлите. Отправляет только экспорт (Telemetry/Diagnostic),
//! принимает команды сеньора (Command/ModeChange/Shutdown). При разрыве канала —
//! store & forward: фреймы копятся в буфере, при восстановлении — drain FIFO.

use std::sync::Arc;

use crate::core::bus::{Bus, BusMessage, BusPayload, DiagLevel};
use crate::core::mode_switch::Priority;
use crate::satellite::satellite_id::SatelliteId;
use crate::storage::store_forward::StoreForward;
use crate::supervisor::PanicCounter;
use crate::transport::frame::Frame;
use crate::transport::peer_link::{LinkError, LinkHealth, PeerLink};

pub struct PeerAgent {
    satellite: SatelliteId,
    link: Box<dyn PeerLink>,
    store: StoreForward,
    bus: Option<Bus>,
    panics: Arc<PanicCounter>,
    store_cap: usize,
}

impl PeerAgent {
    pub fn new(
        satellite: SatelliteId,
        link: Box<dyn PeerLink>,
        store: StoreForward,
        bus: Option<Bus>,
        panics: Arc<PanicCounter>,
    ) -> Self {
        let store_cap = store.capacity();
        Self { satellite, link, store, bus, panics, store_cap }
    }

    pub fn satellite(&self) -> SatelliteId { self.satellite }
    pub fn health(&self) -> LinkHealth { self.link.health() }
    pub fn panics(&self) -> u64 { self.panics.get() }
    pub fn store_cap(&self) -> usize { self.store_cap }

    /// Отправка: если канал жив — напрямую, если лёг — в store & forward.
    pub async fn send(&self, f: Frame) -> Result<(), LinkError> {
        match self.link.health() {
            LinkHealth::Alive | LinkHealth::Degraded => {
                match self.link.send(f.clone()).await {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.diag(DiagLevel::Warning, &format!("send failed: {e}, buffering"));
                        self.store.push(f).await;
                        Err(e)
                    }
                }
            }
            LinkHealth::Down => {
                self.store.push(f).await;
                Ok(())
            }
        }
    }

    /// Восстановление: reconnect + drain FIFO в порядке seq.
    pub async fn reconnect_now(&self) -> Result<usize, LinkError> {
        self.link.reconnect().await?;
        let drained = self.store.drain(self.store_cap).await;
        let n = drained.len();
        for f in drained {
            self.link.send(f).await?;
        }
        if n > 0 {
            self.diag(DiagLevel::Info, &format!("reconnected, drained {n} frames"));
        }
        Ok(n)
    }

    /// Приём: от митрополии (команды сеньора).
    pub async fn recv(&self) -> Result<Frame, LinkError> {
        self.link.recv().await
    }

    fn diag(&self, level: DiagLevel, msg: &str) {
        if let Some(bus) = &self.bus {
            let _ = bus.diagnostics.try_send(BusMessage::new(
                "peer_agent",
                Priority::Hard,
                BusPayload::Diagnostic {
                    level,
                    component: format!("satellite-{}", self.satellite.as_str()),
                    message: msg.to_string(),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::bus::Bus;
    use crate::metrics::OverheadMetrics;
    use crate::satellite::satellite_id::SatelliteId;
    use crate::transport::peer_link::RamFallback;

    fn f(seq: u64) -> Frame {
        Frame::with_seq(seq, SatelliteId::A, SatelliteId::C, vec![1, 2, 3], 1000)
    }

    fn test_agent(link: Box<dyn PeerLink>, cap: usize) -> PeerAgent {
        let m = Arc::new(OverheadMetrics::new());
        let (bus, _rx) = Bus::new(4, 8, 4, 8, m);
        let store = StoreForward::new(cap);
        let panics = Arc::new(PanicCounter::default());
        PeerAgent::new(SatelliteId::A, link, store, Some(bus), panics)
    }

    #[tokio::test]
    async fn send_alive_goes_direct() {
        let (ram, mut rx) = RamFallback::new(8);
        let agent = test_agent(Box::new(ram), 8);
        agent.send(f(1)).await.unwrap();
        assert_eq!(rx.recv().await.unwrap().seq, 1);
    }

    #[tokio::test]
    async fn panics_counter_zero_at_start() {
        let (ram, _rx) = RamFallback::new(8);
        let agent = test_agent(Box::new(ram), 8);
        assert_eq!(agent.panics(), 0);
    }

    #[tokio::test]
    async fn store_cap_reported() {
        let (ram, _rx) = RamFallback::new(8);
        let agent = test_agent(Box::new(ram), 16);
        assert_eq!(agent.store_cap(), 16);
    }

    /// Главный тест: разрыв -> буфер -> heal -> drain FIFO -> паник 0.
    #[tokio::test]
    async fn disconnect_buffers_heal_drains_fifo() {
        let (ram, mut ram_rx) = RamFallback::new(32);
        ram.set_alive(false);
        let agent = test_agent(Box::new(ram), 8);

        // 1) линк лёг - оба фрейма в буфер
        assert_eq!(agent.health(), LinkHealth::Down);
        agent.send(f(1)).await.unwrap();
        agent.send(f(2)).await.unwrap();

        // 2) heal + drain
        let n = agent.reconnect_now().await.unwrap();
        assert_eq!(n, 2, "должно быть 2 фрейма в буфере");

        // 3) порядок seq сохранён
        let g1 = ram_rx.recv().await.unwrap();
        let g2 = ram_rx.recv().await.unwrap();
        assert_eq!(g1.seq, 1);
        assert_eq!(g2.seq, 2);

        // 4) паник - 0
        assert_eq!(agent.panics(), 0);
    }
}