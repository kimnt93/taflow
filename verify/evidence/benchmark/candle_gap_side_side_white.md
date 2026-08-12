# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.73M | 0.016 | 61.29M | 0.047 | 2.46× | 2.86× |
| 10,000 | 0.136 | 73.76M | 0.129 | 77.44M | 0.234 | 1.73× | 1.82× |
| 100,000 | 1.308 | 76.46M | 1.241 | 80.59M | 1.966 | 1.50× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.114 | 1.33× |
| 1 | 5 | 0.354 | 0.452 | 1.28× |
| 1 | 10 | 0.577 | 0.930 | 1.61× |
| 10 | 1 | 0.056 | 0.086 | 1.53× |
| 10 | 5 | 0.263 | 0.427 | 1.62× |
| 10 | 10 | 0.579 | 0.980 | 1.69× |
| 100 | 1 | 0.057 | 0.089 | 1.55× |
| 100 | 5 | 0.257 | 0.442 | 1.72× |
| 100 | 10 | 0.532 | 0.951 | 1.79× |
| 1,000 | 1 | 0.091 | 0.118 | 1.31× |
| 1,000 | 5 | 0.272 | 0.547 | 2.01× |
| 1,000 | 10 | 0.590 | 1.135 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
