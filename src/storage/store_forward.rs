//! src/storage/store_forward.rs - FIFO-буфер на случай разрыва канала.
//!
//! In-memory для теста. Позже — поверх `RawStore` (src/storage/raw.rs).

use std::collections::VecDeque;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::transport::frame::Frame;

#[derive(Debug, Clone)]
pub struct StoreForward {
    inner: Arc<Mutex<VecDeque<Frame>>>,
    capacity: usize,
}

impl StoreForward {
    pub fn new(capacity: usize) -> Self {
        Self { inner: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))), capacity }
    }

    /// Кладёт фрейм в буфер. Если буфер полон — вытесняет самый старый (FIFO).
    pub async fn push(&self, f: Frame) {
        let mut q = self.inner.lock().await;
        if q.len() >= self.capacity {
            q.pop_front();
        }
        q.push_back(f);
    }

    /// Забирает до `max_n` фреймов в порядке FIFO.
    pub async fn drain(&self, max_n: usize) -> Vec<Frame> {
        let mut q = self.inner.lock().await;
        let n = max_n.min(q.len());
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(f) = q.pop_front() { out.push(f); }
        }
        out
    }

    pub async fn len(&self) -> usize { self.inner.lock().await.len() }
    pub async fn is_empty(&self) -> bool { self.inner.lock().await.is_empty() }
    pub fn capacity(&self) -> usize { self.capacity }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::satellite::satellite_id::SatelliteId;

    fn f(seq: u64) -> Frame {
        Frame::with_seq(seq, SatelliteId::A, SatelliteId::C, vec![], 1000)
    }

    #[tokio::test]
    async fn push_drain_fifo() {
        let sf = StoreForward::new(8);
        sf.push(f(1)).await;
        sf.push(f(2)).await;
        sf.push(f(3)).await;
        let out = sf.drain(2).await;
        assert_eq!(out.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(sf.len().await, 1);
    }

    #[tokio::test]
    async fn capacity_evicts_oldest() {
        let sf = StoreForward::new(2);
        sf.push(f(1)).await;
        sf.push(f(2)).await;
        sf.push(f(3)).await;
        assert_eq!(sf.len().await, 2);
        let out = sf.drain(10).await;
        assert_eq!(out.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![2, 3]);
    }

    #[tokio::test]
    async fn drain_more_than_len() {
        let sf = StoreForward::new(8);
        sf.push(f(1)).await;
        let out = sf.drain(10).await;
        assert_eq!(out.len(), 1);
        assert!(sf.is_empty().await);
    }
}

