# IMI

## Summary

`IMI` compares rolling intraday candle gains (`close > open`) with losses. The
state keeps two fixed-capacity windows and rolling sums, so each update is O(1)
and does not rescan history. Python exposes `taflow.talib.IMI` for compatible
batch use and `taflow.IntradayMomentumIndex` for persistent state.

## Correctness and stream mode

- Rust state versus batch: 300 oscillating candles plus reset/replay, pass.
- Both TAFlow Python interfaces versus TA-Lib Python 0.7.1: pass, including a
  20-bar backfill followed by the remaining 280 bars on the same state.
- The 100K benchmark oracle comparison had no warm-up mismatch and maximum
  absolute error 1.42e-14 for both batch and state `extend`.
- A window containing only flat candles returns the neutral value 50.0.
- Unequal open/close lengths and periods below two are rejected.
- Rust implementation: `crates/taflow-core/src/stream/imi.rs`.
- Descriptive Python implementation: `python/taflow/intraday_momentum_index.py`.

## Speed

Criterion `--quick`, after 10,000 warm-up bars and over 1,000,000 Rust-core
updates: 15.03–15.46 ms, approximately 15.1 ns per update.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0093 | 0.0048 | 0.0071 | 0.0325 |
| 1,000 | 0.0188 | 0.0242 | 0.0226 | 0.2472 |
| 10,000 | 0.1494 | 0.2015 | 0.1444 | 2.6166 |
| 100,000 | 1.3709 | 2.0412 | 1.5030 | 26.1454 |
| 1,000,000 | 15.3443 | 20.6597 | 13.9337 | 259.4987 |

At 1M bars, state `extend` delivered 70.5M bars/s, TA-Lib batch delivered
64.3M, and TAFlow batch delivered 48.4M. Their p99 end-to-end latencies were
14.93 ms, 16.16 ms, and 21.73 ms respectively. Peak RSS deltas were 13.14,
13.51, and 13.23 MiB. Scalar Python `append` p99 was 280 ns over 10,000
samples. The shared multi-indicator Pipeline is explicitly unavailable rather
than being conflated with `extend`. All raw samples, CPU times, percentiles,
throughput, memory, and correctness metadata are in `IMI.json`.
