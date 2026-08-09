# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.64M | 0.007 | 139.63M | 0.046 | 5.09× | 6.42× |
| 10,000 | 0.055 | 180.52M | 0.053 | 187.92M | 0.222 | 4.01× | 4.18× |
| 100,000 | 0.676 | 147.92M | 0.548 | 182.55M | 1.898 | 2.81× | 3.47× |
| 1,000,000 | 5.955 | 167.93M | 5.760 | 173.62M | 18.076 | 3.04× | 3.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.144 | 1.44× |
| 1 | 5 | 0.322 | 0.478 | 1.48× |
| 1 | 10 | 0.495 | 0.896 | 1.81× |
| 10 | 1 | 0.054 | 0.089 | 1.64× |
| 10 | 5 | 0.234 | 0.436 | 1.86× |
| 10 | 10 | 0.510 | 0.922 | 1.81× |
| 100 | 1 | 0.055 | 0.092 | 1.67× |
| 100 | 5 | 0.251 | 0.446 | 1.78× |
| 100 | 10 | 0.546 | 0.944 | 1.73× |
| 1,000 | 1 | 0.060 | 0.112 | 1.86× |
| 1,000 | 5 | 0.250 | 0.537 | 2.15× |
| 1,000 | 10 | 0.527 | 1.126 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
