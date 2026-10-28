//! Transport tests (nop, shmem/vsock on Linux only).

use rust_iot_concentrator::transport::nop::NopTransport;
use rust_iot_concentrator::transport::{Transport, TransportMessage};

#[test]
fn nop_send_recv() {
    let t = NopTransport::new();
    t.send(TransportMessage { source: "test".into(), priority: 0, payload: vec![1, 2, 3] }).unwrap();
    let got = t.recv().unwrap();
    assert_eq!(got.source, "test");
    assert_eq!(got.payload, vec![1, 2, 3]);
}

#[test]
fn nop_is_empty() {
    let t = NopTransport::new();
    assert!(t.recv().is_none());
}
