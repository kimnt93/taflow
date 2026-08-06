# SAR

## Summary

`SAR` maintains trend direction, extreme point, acceleration factor, projected
stop, and the previous high/low bar. It performs constant work per update and
requires only a one-bar lookback. Python exposes `taflow.talib.SAR` and
`taflow.ParabolicSar`.

## Correctness and stream mode

- Rust state versus batch: 300 oscillating bars through multiple reversals,
  pass.
- Python versus original TA-Lib 0.7.1: two acceleration configurations with
  extension and reset/replay, pass.
- The first appended bar returns `None`; subsequent values align exactly.
- One implementation file: `crates/taflow-core/src/stream/sar.rs`.
- One descriptive Python file: `python/taflow/parabolic_sar.py`.

## Speed

Criterion `--quick`, 10,000 warm-up bars plus 1,000,000 updates: 8.74–9.07 ms,
approximately 8.8 ns per update in the Rust core.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0039 | 0.0048 | 0.0049 | 0.0298 |
| 1,000 | 0.0117 | 0.0127 | 0.0138 | 0.2508 |
| 10,000 | 0.0514 | 0.0899 | 0.0941 | 2.5237 |
| 100,000 | 0.4898 | 0.8520 | 0.9223 | 23.3956 |
| 1,000,000 | 4.9042 | 8.7217 | 9.5800 | 238.8509 |

At 1M bars TAFlow batch delivered 115.2M bars/s versus TA-Lib's 205.0M;
their p99 latencies were 8.8763 ms and 4.9548 ms. Peak RSS deltas were 13.31
MiB and 13.60 MiB. Scalar Python `append` p99 was 243 ns over 10,000 samples,
and the 100K oracle comparison was exact. Pipeline is not yet available. Full
raw samples and metrics for every size are in `SAR.json`.
