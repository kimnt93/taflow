# TrueStrengthIndex benchmark (`TSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.15M | 0.017 | 58.75M | 0.210 | 11.60× | 12.36× |
| 10,000 | 0.133 | 75.43M | 0.134 | 74.83M | 0.574 | 4.33× | 4.30× |
| 100,000 | 1.349 | 74.11M | 1.369 | 73.02M | 4.532 | 3.36× | 3.31× |
| 1,000,000 | 13.452 | 74.34M | 12.663 | 78.97M | 41.071 | 3.05× | 3.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.313 | 3.92× |
| 1 | 5 | 0.307 | 1.201 | 3.92× |
| 1 | 10 | 0.467 | 2.484 | 5.32× |
| 10 | 1 | 0.054 | 0.238 | 4.39× |
| 10 | 5 | 0.243 | 1.343 | 5.53× |
| 10 | 10 | 0.463 | 2.704 | 5.85× |
| 100 | 1 | 0.052 | 0.248 | 4.77× |
| 100 | 5 | 0.265 | 1.393 | 5.26× |
| 100 | 10 | 0.492 | 2.544 | 5.17× |
| 1,000 | 1 | 0.066 | 0.285 | 4.31× |
| 1,000 | 5 | 0.240 | 1.574 | 6.57× |
| 1,000 | 10 | 0.546 | 3.055 | 5.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
