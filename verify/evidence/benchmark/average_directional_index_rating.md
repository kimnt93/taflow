# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.96M | 0.016 | 62.06M | 0.041 | 2.22× | 2.56× |
| 10,000 | 0.111 | 89.88M | 0.109 | 91.75M | 0.123 | 1.10× | 1.12× |
| 100,000 | 1.054 | 94.86M | 1.048 | 95.39M | 0.965 | 0.92× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.144 | 1.58× |
| 1 | 5 | 0.289 | 0.481 | 1.66× |
| 1 | 10 | 0.538 | 1.005 | 1.87× |
| 10 | 1 | 0.056 | 0.091 | 1.61× |
| 10 | 5 | 0.241 | 0.446 | 1.85× |
| 10 | 10 | 0.555 | 0.967 | 1.74× |
| 100 | 1 | 0.072 | 0.115 | 1.60× |
| 100 | 5 | 0.284 | 0.463 | 1.63× |
| 100 | 10 | 0.517 | 0.969 | 1.88× |
| 1,000 | 1 | 0.066 | 0.124 | 1.87× |
| 1,000 | 5 | 0.287 | 0.519 | 1.81× |
| 1,000 | 10 | 0.540 | 1.058 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
