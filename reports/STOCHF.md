# STOCHF

## Summary

`STOCHF` computes fast %K from the rolling highest high and lowest low, then
smooths %K into fast %D with any of TA-Lib's nine moving-average types. The
state uses monotonic high/low queues and a persistent selected moving average,
so each new bar does bounded work and no full-history recomputation.

Python exposes `taflow.talib.STOCHF` for compatible batch use and
`taflow.FastStochasticOscillator` for persistent state.

## Correctness and stream mode

- Rust batch/state parity over 500 bars for all nine moving-average types:
  pass.
- Both Python surfaces versus TA-Lib Python 0.7.1 for MA types 0 through 8:
  pass.
- The batch implementation now uses the selected MA's true lookback. This
  corrects DEMA, TEMA, KAMA, MAMA, and T3 warm-up placement rather than
  assuming every smoother has a `period - 1` lookback.
- A 20-bar backfill followed by the remaining 480 bars matches a single oracle
  call for every MA type.
- Reset/replay, invalid periods, invalid MA type, and unequal input-length
  checks pass.
- Flat high/low ranges return zero, matching TA-Lib's zero-denominator
  convention.
- Batch, stream, and descriptive wrapper code are isolated in `stochf.rs`,
  `stream/stochf.rs`, and `fast_stochastic_oscillator.py` respectively.

The 100K benchmark correctness comparison had no warm-up mismatch. Maximum
absolute error was 1.12e-12 for TAFlow batch and 4.27e-13 for state `extend`.

## Speed

Criterion `--quick`, after 10,000 warm-up bars and over 1,000,000 Rust-core
updates: 33.07–34.40 ms, approximately 33.3 ns per update.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0059 | 0.0061 | 0.0098 | 0.0382 |
| 1,000 | 0.0116 | 0.0174 | 0.0396 | 0.3257 |
| 10,000 | 0.0743 | 0.1307 | 0.3264 | 3.4570 |
| 100,000 | 0.9777 | 1.8878 | 3.5741 | 33.7675 |
| 1,000,000 | 9.2188 | 20.5328 | 36.5308 | 345.1748 |

At 1M bars TA-Lib batch delivered 107.5M bars/s, TAFlow batch 49.1M, and
state `extend` 27.0M. Their p99 end-to-end latencies were 9.61 ms, 21.52 ms,
and 38.37 ms. Peak RSS deltas were 15.10 MiB, 30.39 MiB, and 20.77 MiB.
Scalar Python `append` p99 was 453 ns over 10,000 samples. Pipeline remains
explicitly unavailable until the shared Rust execution engine exists. Full
samples and metrics are in `STOCHF.json`.
