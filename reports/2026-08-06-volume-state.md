# Volume indicator state validation — 2026-08-06

## Implemented functions

| Function | Stateful method | Warm-up | Complexity |
|---|---|---|---:|
| AD | cumulative close-location value × volume | none | O(1) |
| ADOSC | cumulative AD with first-value-seeded fast and slow EMAs | `max(fast, slow) - 1` | O(1) |
| OBV | signed cumulative volume using previous close | none | O(1) |

All functions expose Rust and Python `append`, `extend`, `value`, and `reset`.

## Correctness evidence

| Test | Series | Result |
|---|---:|---|
| Rust per-bar parity, including zero-range bars | 100 OHLCV bars | pass |
| Python extend and append/reset exact equality | 128 OHLCV bars | pass |
| Complete exhaustive batch plus state suite | 308 tests | pass |
| Rust workspace | 67 tests | pass |

## One-million-update benchmark

Criterion `--quick`, release build, after 10,000 initialization bars. Times
include initialization.

| Function | Total time | Approx. ns/update |
|---|---:|---:|
| AD | 1.59–1.66 ms | 1.6 |
| ADOSC(3,10) | 7.73–7.77 ms | 7.8 |
| OBV | 1.01–1.02 ms | 1.0 |
