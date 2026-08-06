# SAREXT

## Summary

`SAREXT` extends Parabolic SAR with a chosen starting direction/value,
reversal offset, independent long/short acceleration schedules, and signed
output. Python exposes `taflow.talib.SAREXT` and
`taflow.ParabolicSarExtended`.

## Correctness and stream mode

- Rust state versus batch: 300 oscillating bars with asymmetric parameters and
  reversal offset, pass.
- Python versus original TA-Lib 0.7.1: default and asymmetric configurations,
  including reset/replay, pass.
- Positive output denotes long SAR and negative output denotes short SAR.
- One implementation file: `crates/taflow-core/src/stream/sarext.rs`.
- One descriptive Python file: `python/taflow/parabolic_sar_extended.py`.

## Speed

Criterion `--quick`, 10,000 warm-up bars plus 1,000,000 updates: 8.66–8.88 ms,
approximately 8.7 ns per update in the Rust core.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0056 | 0.0057 | 0.0056 | 0.0306 |
| 1,000 | 0.0097 | 0.0127 | 0.0139 | 0.2344 |
| 10,000 | 0.0533 | 0.0920 | 0.0947 | 2.6454 |
| 100,000 | 0.4944 | 0.8850 | 0.9288 | 23.5974 |
| 1,000,000 | 5.0122 | 8.7654 | 8.9655 | 248.9894 |

At 1M bars TAFlow batch delivered 108.2M bars/s versus TA-Lib's 197.9M;
their p99 latencies were 10.7308 ms and 5.2278 ms. Peak RSS deltas were 13.34
MiB and 13.60 MiB. Scalar Python `append` p99 was 247 ns over 10,000 samples,
and the 100K oracle comparison was exact. Pipeline is not yet available. Full
raw samples and metrics for every size are in `SAREXT.json`.
