# RollingZScore benchmark (`ZScore` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.389 | 2.57M | 0.390 | 2.57M | 0.165 | 0.42× | 0.42× |
| 10,000 | 3.772 | 2.65M | 3.766 | 2.66M | 0.510 | 0.14× | 0.14× |
| 100,000 | 38.008 | 2.63M | 37.671 | 2.65M | 5.063 | 0.13× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.253 | 2.61× |
| 1 | 5 | 0.409 | 1.311 | 3.21× |
| 1 | 10 | 0.616 | 2.238 | 3.63× |
| 10 | 1 | 0.080 | 0.215 | 2.69× |
| 10 | 5 | 0.287 | 1.214 | 4.23× |
| 10 | 10 | 0.625 | 2.235 | 3.57× |
| 100 | 1 | 0.111 | 0.217 | 1.95× |
| 100 | 5 | 0.296 | 1.267 | 4.28× |
| 100 | 10 | 0.618 | 2.264 | 3.66× |
| 1,000 | 1 | 0.467 | 0.250 | 0.54× |
| 1,000 | 5 | 0.835 | 1.433 | 1.72× |
| 1,000 | 10 | 1.167 | 2.616 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
