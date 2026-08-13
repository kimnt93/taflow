# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.89M | 0.076 | 13.22M | 0.035 | 0.41× | 0.46× |
| 10,000 | 0.666 | 15.02M | 0.672 | 14.88M | 0.122 | 0.18× | 0.18× |
| 100,000 | 6.805 | 14.70M | 6.319 | 15.83M | 0.948 | 0.14× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.103 | 1.05× |
| 1 | 5 | 0.425 | 0.481 | 1.13× |
| 1 | 10 | 0.660 | 0.901 | 1.37× |
| 10 | 1 | 0.075 | 0.092 | 1.23× |
| 10 | 5 | 0.311 | 0.423 | 1.36× |
| 10 | 10 | 0.651 | 0.906 | 1.39× |
| 100 | 1 | 0.077 | 0.089 | 1.15× |
| 100 | 5 | 0.335 | 0.416 | 1.24× |
| 100 | 10 | 0.666 | 0.915 | 1.37× |
| 1,000 | 1 | 0.138 | 0.099 | 0.72× |
| 1,000 | 5 | 0.337 | 0.464 | 1.38× |
| 1,000 | 10 | 0.637 | 1.055 | 1.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
