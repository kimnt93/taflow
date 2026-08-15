# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.02M | 0.029 | 34.79M | 0.951 | 31.41× | 33.10× |
| 10,000 | 0.413 | 24.20M | 0.309 | 32.39M | 9.888 | 23.93× | 32.03× |
| 100,000 | 2.840 | 35.21M | 2.588 | 38.64M | 72.329 | 25.47× | 27.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.271 | 2.90× |
| 1 | 5 | 0.231 | 1.352 | 5.84× |
| 1 | 10 | 0.463 | 2.778 | 6.00× |
| 10 | 1 | 0.052 | 0.253 | 4.91× |
| 10 | 5 | 0.223 | 1.347 | 6.05× |
| 10 | 10 | 0.523 | 2.582 | 4.94× |
| 100 | 1 | 0.062 | 0.308 | 5.00× |
| 100 | 5 | 0.227 | 1.856 | 8.16× |
| 100 | 10 | 0.470 | 3.172 | 6.75× |
| 1,000 | 1 | 0.074 | 1.006 | 13.53× |
| 1,000 | 5 | 0.233 | 5.098 | 21.84× |
| 1,000 | 10 | 0.499 | 10.366 | 20.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
