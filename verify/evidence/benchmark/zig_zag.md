# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.83M | 0.010 | 96.86M | 0.487 | 27.18× | 47.16× |
| 10,000 | 0.081 | 124.05M | 0.076 | 130.81M | 3.396 | 42.13× | 44.43× |
| 100,000 | 0.824 | 121.31M | 0.747 | 133.86M | 49.384 | 59.91× | 66.10× |
| 1,000,000 | 8.975 | 111.42M | 8.615 | 116.07M | 410.079 | 45.69× | 47.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.266 | 2.63× |
| 1 | 5 | 0.305 | 1.109 | 3.64× |
| 1 | 10 | 0.506 | 2.285 | 4.51× |
| 10 | 1 | 0.051 | 0.231 | 4.55× |
| 10 | 5 | 0.226 | 1.267 | 5.61× |
| 10 | 10 | 0.501 | 2.352 | 4.70× |
| 100 | 1 | 0.052 | 0.256 | 4.94× |
| 100 | 5 | 0.231 | 1.436 | 6.21× |
| 100 | 10 | 0.502 | 2.716 | 5.41× |
| 1,000 | 1 | 0.058 | 0.717 | 12.41× |
| 1,000 | 5 | 0.233 | 3.112 | 13.34× |
| 1,000 | 10 | 0.516 | 6.268 | 12.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
