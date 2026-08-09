# KalmanHedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.25M | 0.017 | 59.25M | nan | — | — |
| 10,000 | 0.143 | 69.96M | 0.140 | 71.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.263 | 0.193 | 5.17M | nan | — | — |
| 1,500 | 10 | 1.517 | 0.829 | 12.07M | nan | — | — |
| 1,500 | 100 | 3.535 | 2.654 | 37.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.65M | 13.45M | 1.00× | 1.05M | 1.46M | 1.00× | — |
| 2 | 18.92M | 20.67M | 1.54× | 1.22M | 1.59M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
