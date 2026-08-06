# Rolling OHLC momentum state validation — 2026-08-06

## Implemented functions

| Function | Stateful method | Warm-up | Complexity |
|---|---|---|---:|
| BOP | current-bar `(close-open)/(high-low)` | none | O(1) |
| WILLR | monotonic high maximum and low minimum queues | period − 1 | amortized O(1) |
| AROON | latest-extremum indices over `period + 1` bars | period | amortized O(1) |
| AROONOSC | fused Aroon up minus down | period | amortized O(1) |
| CMO | seeded up/down changes with Wilder recurrence | period | O(1) |

All functions expose Rust and Python `append`, `extend`, `value`, and `reset`.
AROON returns aligned down/up arrays in the same order as TA-Lib.

## Correctness evidence

| Test | Series | Result |
|---|---:|---|
| Rust per-bar parity with flat-range bars | 100 OHLC bars, period 14 | pass |
| Python extend and append/reset exact equality | 128 OHLC bars, period 14 | pass |
| CMO Rust/Python per-bar and append/reset parity | 128 values, period 14 | pass |
| Complete exhaustive batch plus state suite | 313 tests | pass |
| Rust workspace | 68 tests | pass |

## One-million-update benchmark

Criterion `--quick`, release build, after 10,000 initialization bars. Times
include initialization.

| Function | Total time | Approx. ns/update |
|---|---:|---:|
| BOP | 1.18–1.23 ms | 1.2 |
| WILLR(14) | 25.88–26.64 ms | 26.0 |
| AROON(14) | 26.92–26.98 ms | 27.0 |
| AROONOSC(14) | 27.90–27.97 ms | 28.0 |
| CMO(20) | 5.67–5.78 ms | 5.7 |

CMO is implemented in its own English-documented `stream/cmo.rs` module and
re-exported explicitly from `stream/mod.rs`, establishing the required source
layout for all subsequent TA state functions.
