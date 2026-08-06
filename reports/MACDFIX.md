# MACDFIX

## Summary

`MACDFIX` is TA-Lib's fixed 12/26 Moving Average Convergence/Divergence
variant. Its fast and slow recurrences use the fixed constants 0.15 and 0.075,
not ordinary MACD's period-derived constants. The state performs constant work
per appended close and retains only its 26-value seed buffer plus EMA state.

Python exposes `taflow.talib.MACDFIX` for compatible batch use and
`taflow.MovingAverageConvergenceDivergenceFixed` for persistent state.

## Correctness and stream mode

- Rust state versus the verified batch implementation: 300 values, pass.
- Both Python surfaces versus TA-Lib Python 0.7.1 for signal periods 1, 5, and
  9: pass.
- A separate comparison proves MACDFIX differs from ordinary MACD on the same
  series, confirming that the fixed constants are used.
- A 20-value backfill followed by the remaining 280 values on the same state
  matches the single-call oracle output.
- Reset/replay and signal-period-zero rejection pass.
- The batch implementation is isolated in
  `crates/taflow-core/src/momentum/macdfix.rs`. MACD and MACDFIX states are isolated in
  `crates/taflow-core/src/stream/macd.rs` and `macdfix.rs`; `stream/mod.rs`
  contains only their declarations and re-exports.
- The descriptive wrapper is isolated in
  `python/taflow/moving_average_convergence_divergence_fixed.py`.

The 100K benchmark correctness comparison had no warm-up mismatch and maximum
absolute error 5.68e-14 for both TAFlow batch and state `extend`.

## Speed

Criterion `--quick`, after 10,000 warm-up values and over 1,000,000 Rust-core
updates: 10.34–10.81 ms, approximately 10.4 ns per update.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 values and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0045 | 0.0042 | 0.0054 | 0.0323 |
| 1,000 | 0.0139 | 0.0110 | 0.0181 | 0.2361 |
| 10,000 | 0.0961 | 0.0822 | 0.1415 | 2.5297 |
| 100,000 | 1.6146 | 1.4046 | 1.7299 | 23.2588 |
| 1,000,000 | 16.9269 | 14.2235 | 17.4751 | 236.5322 |

At 1M values TAFlow batch delivered 70.7M bars/s versus TA-Lib's 59.4M. Their
p99 end-to-end latencies were 14.54 ms and 17.48 ms. State `extend` delivered
56.9M bars/s with 18.98 ms p99. Peak RSS deltas were 30.34 MiB, 22.78 MiB, and
15.14 MiB respectively. Scalar Python `append` p99 was 278 ns over 10,000
samples. Pipeline remains explicitly unavailable until the shared Rust
execution engine exists. Full samples and metrics are in `MACDFIX.json`.
