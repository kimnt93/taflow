# STOCH

## Summary

`STOCH` calculates fast %K from the rolling highest high and lowest low,
smooths it into slow %K, and smooths slow %K into slow %D. Both smoothers can
use any of TA-Lib's nine moving-average types. The state keeps monotonic
extrema queues and two persistent moving averages, avoiding history rescans.

Python exposes `taflow.talib.STOCH` for compatible batch use and
`taflow.StochasticOscillator` for persistent state.

## Correctness and stream mode

- Rust batch/state parity over 500 bars for all 81 slow-K/slow-D MA pairs:
  pass.
- Both Python surfaces versus TA-Lib Python 0.7.1 for the same 81 pairs: pass.
- The inherited implementation failed 65 of those 81 pairs. Batch and stream
  now use both selected MAs' true lookbacks and align slow %K to slow %D.
- Flat high/low ranges return zero, matching TA-Lib's zero-denominator rule.
- A 20-bar backfill followed by the remaining 480 bars matches a single oracle
  call for every pair.
- Reset/replay, invalid periods, invalid MA types, and unequal input lengths
  pass.
- STOCH and STOCHRSI batch code are now isolated in separate `stoch.rs` and
  `stochrsi.rs` files. Streaming and the descriptive wrapper are isolated in
  `stream/stoch.rs` and `stochastic_oscillator.py`.

The 100K benchmark correctness comparison had no warm-up mismatch. Maximum
absolute error was 2.63e-12 for TAFlow batch and 2.27e-12 for state `extend`.

## Speed

Criterion `--quick`, after 10,000 warm-up bars and over 1,000,000 Rust-core
updates: 36.78–37.18 ms, approximately 36.9 ns per update.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0109 | 0.0054 | 0.0104 | 0.0411 |
| 1,000 | 0.0196 | 0.0185 | 0.0450 | 0.3576 |
| 10,000 | 0.0927 | 0.0987 | 0.3822 | 3.5315 |
| 100,000 | 1.1552 | 1.7722 | 4.0765 | 35.0100 |
| 1,000,000 | 11.8824 | 18.0181 | 43.3503 | 366.9496 |

At 1M bars TA-Lib batch delivered 81.0M bars/s, TAFlow batch 54.8M, and
state `extend` 22.7M. Their p99 end-to-end latencies were 14.32 ms, 18.73 ms,
and 46.30 ms. Peak RSS deltas were 15.01 MiB, 37.93 MiB, and 20.57 MiB.
Scalar Python `append` p99 was 575 ns over 10,000 samples. Pipeline remains
explicitly unavailable until the shared Rust execution engine exists. Full
samples and metrics are in `STOCH.json`.
