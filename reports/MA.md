# MA

## Summary

`MA` selects SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA, MAMA, or T3. The Rust
stream state dispatches each appended value to the concrete state without
recomputing history. Python exposes `taflow.talib.MA` for batch compatibility
and `taflow.MovingAverage` for persistent streaming.

## Correctness and stream mode

- Rust state versus batch: 200 values for each of 9 MA types, pass.
- Python state versus original TA-Lib 0.7.1: `extend` and reset/replay for all
  9 MA types, pass.
- Warm-up is inherited from the selected MA, including MAMA and T3.
- KAMA now matches TA-Lib's epsilon/update-order semantics and treats period
  one as the required identity transform.
- One implementation file: `crates/taflow-core/src/stream/ma.rs`.
- One descriptive Python file: `python/taflow/moving_average.py`.

## Speed

Criterion `--quick`, after 10,000 warm-up values and over 1,000,000 updates:
MA(20, EMA) completed in 4.84–5.01 ms, approximately 5.0 ns per update. This is
a Rust-core stream measurement; Python call overhead is not included.

Five isolated-process runs per cell; values below are p50 wall milliseconds.
Streaming first backfills 10,000 bars and then times the stated append count.

| Bars | TA-Lib batch | TAFlow batch | State `extend` | Streaming append loop |
|---:|---:|---:|---:|---:|
| 100 | 0.0053 | 0.0046 | 0.0072 | 0.0250 |
| 1,000 | 0.0086 | 0.0073 | 0.0098 | 0.2168 |
| 10,000 | 0.0302 | 0.0251 | 0.0630 | 2.0970 |
| 100,000 | 0.2767 | 0.2376 | 0.5942 | 20.0535 |
| 1,000,000 | 2.8552 | 2.3062 | 6.2564 | 204.4443 |

At 1M bars TAFlow batch delivered 429.7M bars/s versus TA-Lib's 346.6M;
their p99 latencies were 2.3743 ms and 3.1913 ms. Peak RSS deltas were 13.25
MiB and 13.56 MiB. Scalar Python `append` p99 was 241 ns over 10,000 samples.
The Pipeline result is explicitly unavailable until the shared Rust execution
engine exists. Full raw samples, CPU time, p95/p99/max, throughput, and memory
for every size are in `MA.json`.
