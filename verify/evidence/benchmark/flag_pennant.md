# FlagPennant benchmark (`FlagPennant` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.08M | 0.006 | 159.74M | 0.238 | 28.30× | 37.96× |
| 10,000 | 0.085 | 117.85M | 0.080 | 124.98M | 1.323 | 15.60× | 16.54× |
| 100,000 | 0.787 | 127.10M | 0.749 | 133.53M | 12.970 | 16.48× | 17.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.214 | 3.05× |
| 1 | 5 | 0.311 | 0.805 | 2.59× |
| 1 | 10 | 0.408 | 1.700 | 4.16× |
| 10 | 1 | 0.042 | 0.166 | 3.97× |
| 10 | 5 | 0.209 | 1.084 | 5.19× |
| 10 | 10 | 0.397 | 1.737 | 4.37× |
| 100 | 1 | 0.046 | 0.179 | 3.91× |
| 100 | 5 | 0.199 | 1.118 | 5.63× |
| 100 | 10 | 0.435 | 1.876 | 4.31× |
| 1,000 | 1 | 0.052 | 0.290 | 5.61× |
| 1,000 | 5 | 0.245 | 1.808 | 7.39× |
| 1,000 | 10 | 0.491 | 2.988 | 6.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
