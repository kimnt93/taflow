# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.30M | 0.014 | 70.27M | 0.034 | 2.16× | 2.39× |
| 10,000 | 0.148 | 67.71M | 0.134 | 74.51M | 0.148 | 1.00× | 1.10× |
| 100,000 | 1.368 | 73.08M | 1.351 | 74.03M | 1.277 | 0.93× | 0.95× |
| 1,000,000 | 14.771 | 67.70M | 13.949 | 71.69M | 13.114 | 0.89× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.112 | 1.15× |
| 1 | 5 | 0.379 | 0.510 | 1.34× |
| 1 | 10 | 0.529 | 0.919 | 1.74× |
| 10 | 1 | 0.054 | 0.092 | 1.70× |
| 10 | 5 | 0.239 | 0.425 | 1.77× |
| 10 | 10 | 0.532 | 0.875 | 1.64× |
| 100 | 1 | 0.055 | 0.088 | 1.60× |
| 100 | 5 | 0.278 | 0.437 | 1.57× |
| 100 | 10 | 0.511 | 0.893 | 1.75× |
| 1,000 | 1 | 0.073 | 0.107 | 1.48× |
| 1,000 | 5 | 0.261 | 0.499 | 1.91× |
| 1,000 | 10 | 0.594 | 1.017 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
