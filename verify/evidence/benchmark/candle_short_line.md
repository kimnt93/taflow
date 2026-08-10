# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.92M | 0.019 | 51.33M | 0.048 | 2.08× | 2.49× |
| 10,000 | 0.212 | 47.14M | 0.197 | 50.66M | 0.263 | 1.24× | 1.33× |
| 100,000 | 2.192 | 45.61M | 2.021 | 49.47M | 2.231 | 1.02× | 1.10× |
| 1,000,000 | 19.871 | 50.32M | 18.715 | 53.43M | 20.211 | 1.02× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.125 | 1.18× |
| 1 | 5 | 0.416 | 0.672 | 1.62× |
| 1 | 10 | 0.717 | 1.233 | 1.72× |
| 10 | 1 | 0.095 | 0.138 | 1.45× |
| 10 | 5 | 0.449 | 0.579 | 1.29× |
| 10 | 10 | 0.743 | 1.302 | 1.75× |
| 100 | 1 | 0.082 | 0.126 | 1.53× |
| 100 | 5 | 0.379 | 0.606 | 1.60× |
| 100 | 10 | 0.853 | 1.419 | 1.66× |
| 1,000 | 1 | 0.106 | 0.143 | 1.35× |
| 1,000 | 5 | 0.381 | 0.778 | 2.04× |
| 1,000 | 10 | 0.810 | 1.455 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
