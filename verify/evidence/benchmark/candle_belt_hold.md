# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.22M | 0.016 | 62.70M | 0.040 | 2.10× | 2.48× |
| 10,000 | 0.156 | 63.91M | 0.155 | 64.61M | 0.137 | 0.88× | 0.89× |
| 100,000 | 1.598 | 62.59M | 1.575 | 63.48M | 1.096 | 0.69× | 0.70× |
| 1,000,000 | 16.015 | 62.44M | 15.723 | 63.60M | 10.414 | 0.65× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.117 | 0.86× |
| 1 | 5 | 0.284 | 0.462 | 1.62× |
| 1 | 10 | 0.502 | 0.981 | 1.96× |
| 10 | 1 | 0.067 | 0.098 | 1.47× |
| 10 | 5 | 0.256 | 0.440 | 1.72× |
| 10 | 10 | 0.551 | 0.908 | 1.65× |
| 100 | 1 | 0.056 | 0.093 | 1.66× |
| 100 | 5 | 0.295 | 0.523 | 1.77× |
| 100 | 10 | 0.579 | 0.934 | 1.61× |
| 1,000 | 1 | 0.071 | 0.103 | 1.45× |
| 1,000 | 5 | 0.284 | 0.519 | 1.83× |
| 1,000 | 10 | 0.668 | 1.130 | 1.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
