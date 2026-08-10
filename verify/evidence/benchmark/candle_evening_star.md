# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.21M | 0.017 | 58.33M | 0.037 | 1.84× | 2.13× |
| 10,000 | 0.142 | 70.62M | 0.136 | 73.49M | 0.110 | 0.78× | 0.81× |
| 100,000 | 1.373 | 72.85M | 1.384 | 72.27M | 0.827 | 0.60× | 0.60× |
| 1,000,000 | 13.829 | 72.31M | 13.727 | 72.85M | 8.607 | 0.62× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.109 | 1.39× |
| 1 | 5 | 0.377 | 0.472 | 1.25× |
| 1 | 10 | 0.526 | 0.957 | 1.82× |
| 10 | 1 | 0.054 | 0.100 | 1.86× |
| 10 | 5 | 0.248 | 0.455 | 1.84× |
| 10 | 10 | 0.532 | 0.963 | 1.81× |
| 100 | 1 | 0.055 | 0.097 | 1.77× |
| 100 | 5 | 0.252 | 0.444 | 1.76× |
| 100 | 10 | 0.551 | 0.948 | 1.72× |
| 1,000 | 1 | 0.069 | 0.104 | 1.51× |
| 1,000 | 5 | 0.273 | 0.526 | 1.93× |
| 1,000 | 10 | 0.568 | 1.060 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
