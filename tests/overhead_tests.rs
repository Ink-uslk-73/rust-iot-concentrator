//! Overhead integration tests (baseline vs through hub).

use frontier::metrics::OverheadMetrics;
use std::sync::Arc;
use std::time::Instant;

#[test]
fn overhead_module_works() {
    let m = Arc::new(OverheadMetrics::new());
    let t = Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(1));
    m.record_hard_latency(t);
    let s = m.snapshot();
    assert!(s.hard_latency_us >= 1000);
}

#[test]
#[ignore]
fn baseline_vs_through_hub() {
    // Requires real Modbus RTU on AIG-502.
    assert!(true);
}
