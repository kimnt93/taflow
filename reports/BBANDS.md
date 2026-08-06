# BBANDS

## Summary

`BBANDS` produces upper, middle, and lower Bollinger Bands. The selected MA
type controls only the middle band; TA-Lib's population deviation remains
centered on the rolling SMA. Python exposes `taflow.talib.BBANDS` and the
stateful descriptive `taflow.BollingerBands` class.

## Correctness and stream mode

- Rust state versus batch: 200 values for each of 9 MA types, pass.
- Python state versus original TA-Lib 0.7.1: all 9 MA types plus reset/replay,
  pass.
- Multi-output order is upper, middle, lower with aligned `NaN` warm-up.
- One implementation file: `crates/taflow-core/src/stream/bbands.rs`.
- One descriptive Python file: `python/taflow/bollinger_bands.py`.

## Speed

Criterion `--quick`, 10,000 warm-up values plus 1,000,000 updates:
BBANDS(20, 2, 2, SMA) completed in 17.26–17.65 ms, approximately 17.3 ns per
update. This is a Rust-core measurement.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0048 | 0.0070 | 0.0083 | 0.0307 |
| 1,000 | 0.0142 | 0.0078 | 0.0246 | 0.2490 |
| 10,000 | 0.0534 | 0.0380 | 0.1957 | 2.4567 |
| 100,000 | 0.5115 | 0.3389 | 2.0256 | 23.6425 |
| 1,000,000 | 6.6298 | 6.4903 | 23.0429 | 247.6450 |

At 1M bars TAFlow batch delivered 154.2M bars/s versus TA-Lib's 150.5M;
their p99 latencies were 6.7009 ms and 6.9482 ms. Peak RSS deltas were 15.27
MiB and 15.21 MiB. Scalar Python `append` p99 was 754 ns over 10,000 samples.
The 100K oracle comparison passed at `rtol=1e-8`, with maximum absolute error
1.06e-7 from rolling accumulation order. Pipeline is not yet available. Full
raw samples and metrics for every size are in `BBANDS.json`.
