# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.41M | 0.009 | 115.20M | 0.030 | 2.60× | 3.51× |
| 10,000 | 0.074 | 135.09M | 0.071 | 139.89M | 0.089 | 1.20× | 1.24× |
| 100,000 | 0.826 | 121.07M | 0.891 | 112.23M | 0.749 | 0.91× | 0.84× |
| 1,000,000 | 9.614 | 104.02M | 8.364 | 119.56M | 6.493 | 0.68× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.155 | 2.00× |
| 1 | 5 | 0.349 | 0.491 | 1.41× |
| 1 | 10 | 0.601 | 1.050 | 1.75× |
| 10 | 1 | 0.060 | 0.096 | 1.60× |
| 10 | 5 | 0.295 | 0.470 | 1.59× |
| 10 | 10 | 0.526 | 0.901 | 1.71× |
| 100 | 1 | 0.053 | 0.091 | 1.72× |
| 100 | 5 | 0.254 | 0.510 | 2.01× |
| 100 | 10 | 0.586 | 0.951 | 1.62× |
| 1,000 | 1 | 0.068 | 0.095 | 1.41× |
| 1,000 | 5 | 0.254 | 0.464 | 1.83× |
| 1,000 | 10 | 0.664 | 1.107 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
