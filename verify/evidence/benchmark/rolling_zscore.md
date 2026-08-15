# RollingZScore benchmark (`ZScore` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.31M | 0.030 | 33.04M | 0.197 | 6.16× | 6.50× |
| 10,000 | 0.299 | 33.44M | 0.390 | 25.65M | 0.610 | 2.04× | 1.56× |
| 100,000 | 2.871 | 34.83M | 3.029 | 33.02M | 4.613 | 1.61× | 1.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.244 | 2.63× |
| 1 | 5 | 0.273 | 1.293 | 4.73× |
| 1 | 10 | 0.395 | 2.380 | 6.02× |
| 10 | 1 | 0.045 | 0.241 | 5.41× |
| 10 | 5 | 0.202 | 1.234 | 6.12× |
| 10 | 10 | 0.433 | 2.411 | 5.57× |
| 100 | 1 | 0.052 | 0.211 | 4.05× |
| 100 | 5 | 0.197 | 1.235 | 6.28× |
| 100 | 10 | 0.411 | 2.305 | 5.61× |
| 1,000 | 1 | 0.074 | 0.245 | 3.33× |
| 1,000 | 5 | 0.207 | 1.385 | 6.69× |
| 1,000 | 10 | 0.475 | 2.607 | 5.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
