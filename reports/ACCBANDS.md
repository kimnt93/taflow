# ACCBANDS

## Summary

`ACCBANDS` applies TA-Lib's acceleration transform to high and low, then
advances three rolling SMAs for upper, middle, and lower bands. Python exposes
`taflow.talib.ACCBANDS` and `taflow.AccelerationBands`.

## Correctness and stream mode

- Rust state versus batch: 200 OHLC values plus reset/replay, pass.
- Python state and both TAFlow interfaces versus original TA-Lib 0.7.1: pass.
- `append(high, low, close)` returns `None` for the first `period - 1` bars.
- `extend` returns three aligned arrays in upper, middle, lower order.
- One implementation file: `crates/taflow-core/src/stream/accbands.rs`.
- One descriptive Python file: `python/taflow/acceleration_bands.py`.

## Speed

Criterion `--quick`, 10,000 warm-up bars plus 1,000,000 updates: 16.93–17.06
ms, approximately 17.0 ns per update. The measurement includes all three
rolling bands in Rust and excludes Python call overhead.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0050 | 0.0050 | 0.0070 | 0.0427 |
| 1,000 | 0.0129 | 0.0097 | 0.0228 | 0.3414 |
| 10,000 | 0.0845 | 0.0695 | 0.1870 | 3.1645 |
| 100,000 | 1.4065 | 0.6858 | 1.8287 | 32.3894 |
| 1,000,000 | 12.6259 | 9.4044 | 22.9001 | 329.3832 |

At 1M bars TAFlow batch delivered 106.6M bars/s versus TA-Lib's 77.8M;
their p99 latencies were 9.7160 ms and 13.4609 ms. Peak RSS deltas were 15.27
MiB and 30.53 MiB. Scalar Python `append` p99 was 311 ns over 10,000 samples,
and the 100K oracle comparison was exact. Pipeline is not yet available. Full
raw samples and metrics for every size are in `ACCBANDS.json`.
