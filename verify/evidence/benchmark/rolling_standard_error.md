# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.30M | 0.032 | 30.89M | 0.198 | 5.61× | 6.13× |
| 10,000 | 0.300 | 33.31M | 0.300 | 33.29M | 0.753 | 2.51× | 2.51× |
| 100,000 | 3.256 | 30.71M | 2.952 | 33.88M | 6.317 | 1.94× | 2.14× |
| 1,000,000 | 33.476 | 29.87M | 35.677 | 28.03M | 58.026 | 1.73× | 1.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.293 | 3.79× |
| 1 | 5 | 0.369 | 1.242 | 3.37× |
| 1 | 10 | 0.490 | 2.499 | 5.10× |
| 10 | 1 | 0.064 | 0.232 | 3.60× |
| 10 | 5 | 0.256 | 1.250 | 4.88× |
| 10 | 10 | 0.504 | 2.625 | 5.21× |
| 100 | 1 | 0.059 | 0.216 | 3.65× |
| 100 | 5 | 0.226 | 1.293 | 5.72× |
| 100 | 10 | 0.593 | 2.520 | 4.25× |
| 1,000 | 1 | 0.088 | 0.280 | 3.18× |
| 1,000 | 5 | 0.272 | 1.637 | 6.03× |
| 1,000 | 10 | 0.544 | 2.992 | 5.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
