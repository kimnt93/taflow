# RogersSatchell benchmark (`RogersSatchellVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.81M | 0.035 | 28.65M | 0.332 | 8.57× | 9.51× |
| 10,000 | 0.307 | 32.60M | 0.296 | 33.79M | 1.602 | 5.22× | 5.42× |
| 100,000 | 2.751 | 36.35M | 2.782 | 35.94M | 14.406 | 5.24× | 5.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.267 | 2.11× |
| 1 | 5 | 0.408 | 1.421 | 3.48× |
| 1 | 10 | 0.531 | 2.575 | 4.85× |
| 10 | 1 | 0.060 | 0.250 | 4.19× |
| 10 | 5 | 0.257 | 1.579 | 6.15× |
| 10 | 10 | 0.543 | 2.945 | 5.43× |
| 100 | 1 | 0.065 | 0.263 | 4.04× |
| 100 | 5 | 0.266 | 1.566 | 5.89× |
| 100 | 10 | 0.566 | 2.769 | 4.89× |
| 1,000 | 1 | 0.090 | 0.407 | 4.50× |
| 1,000 | 5 | 0.280 | 2.272 | 8.11× |
| 1,000 | 10 | 0.577 | 4.356 | 7.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
