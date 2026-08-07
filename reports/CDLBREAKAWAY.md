# CDLBREAKAWAY — benchmark vs TA-Lib 0.7.1

Correctness: **PASS** @100,000 bars — batch vs talib: max abs err 0.00e+00, 0 NaN mismatches

## Bulk compute (mean seconds per call, 20 repeats)

| Bars | TA-Lib ms | TAFlow ms | Speedup | TAFlow ops/s | State-cold ms | Speedup |
|---:|---:|---:|---:|---:|---:|---:|
| 100 | 0.0010 | 0.0012 | 0.83× | 84.6M | — | — |
| 1,000 | 0.0030 | 0.0032 | 0.94× | 315.4M | — | — |
| 10,000 | 0.0589 | 0.0912 | 0.65× | 109.6M | — | — |
| 100,000 | 0.6505 | 1.1662 | 0.56× | 85.7M | — | — |
| 1,000,000 | 7.1792 | 12.1812 | 0.59× | 82.1M | — | — |

## Parallel continuation (100,000-bar warmed history per thread, one independent stream per thread)

| Threads | TAFlow agg updates/s | Scaling | TA-Lib agg updates/s | Scaling | Speedup |
|---:|---:|---:|---:|---:|---:|
| 1 | — | — | 1.5K | 1.00× | — |
| 2 | — | — | 1.5K | 1.00× | — |
| 5 | — | — | 1.5K | 1.01× | — |
| 10 | — | — | 1.5K | 1.00× | — |
| 20 | — | — | 1.4K | 0.95× | — |

Each thread owns its own state/stream (N-symbol live feed model). Scaling >1× with threads requires the underlying call to release the GIL.

---
Python-interface measurement: numbers include conversion/boundary overhead by design. Rust-core-only numbers live in criterion benches and are not comparable.
