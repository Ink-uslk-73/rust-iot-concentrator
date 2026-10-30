# Overhead metrics

**Статус:** accepted
**Связанные:** [ADR-0011](../decisions/ADR-0011-tablet-first-telemetry.md)

`GET /api/v1/metrics/overhead`

## Thresholds

| Metric | Threshold | Status |
|--------|-----------|--------|
| hard_latency_us | > 1000 (1 ms) | degraded |
| hard_latency_us | > 5000 (5 ms) | critical |
| api_latency_us | > 1000 (1 ms) | degraded |
| sse_jitter_ms | > 10 | degraded |
| bus_buffer_pct | > 70 | degraded |
| bus_buffer_pct | > 90 | critical |
| bus_drops | > 0 | critical |

## Where measured

| Metric | File |
|--------|------|
| hard_latency_us | src/protocols/hard/modbus.rs (poll_port) |
| api_latency_us | src/api/baselines.rs, events.rs |
| sse_jitter_ms | src/api/events.rs (stream) |
| bus_buffer_pct | src/core/bus.rs (send, try_send) |
| bus_drops | src/core/bus.rs (send, try_send) |
