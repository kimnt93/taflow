# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.19M | 0.008 | 119.86M | 0.032 | 2.61× | 3.85× |
| 10,000 | 0.063 | 157.67M | 0.058 | 172.15M | 0.079 | 1.24× | 1.35× |
| 100,000 | 0.610 | 163.85M | 0.599 | 167.01M | 0.524 | 0.86× | 0.88× |
| 1,000,000 | 7.367 | 135.74M | 8.211 | 121.78M | 6.931 | 0.94× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.137 | 1.14× |
| 1 | 5 | 0.729 | 0.963 | 1.32× |
| 1 | 10 | 0.860 | 2.245 | 2.61× |
| 10 | 1 | 0.090 | 0.289 | 3.20× |
| 10 | 5 | 0.596 | 1.088 | 1.82× |
| 10 | 10 | 0.634 | 1.092 | 1.72× |
| 100 | 1 | 0.070 | 0.091 | 1.30× |
| 100 | 5 | 0.296 | 1.986 | 6.72× |
| 100 | 10 | 0.751 | 1.235 | 1.64× |
| 1,000 | 1 | 0.072 | 0.138 | 1.91× |
| 1,000 | 5 | 0.376 | 0.652 | 1.73× |
| 1,000 | 10 | 0.689 | 1.059 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
