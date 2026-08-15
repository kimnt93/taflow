# SmoothedTrendChannel benchmark (`smoothed trend channel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.23M | 0.011 | 94.72M | 0.577 | 42.25× | 54.65× |
| 10,000 | 0.120 | 83.38M | 0.115 | 86.60M | 5.002 | 41.71× | 43.32× |
| 100,000 | 1.240 | 80.63M | 1.276 | 78.36M | 49.135 | 39.62× | 38.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.198 | 2.91× |
| 1 | 5 | 0.300 | 0.585 | 1.95× |
| 1 | 10 | 0.413 | 1.051 | 2.54× |
| 10 | 1 | 0.043 | 0.183 | 4.24× |
| 10 | 5 | 0.190 | 0.866 | 4.56× |
| 10 | 10 | 0.408 | 1.718 | 4.22× |
| 100 | 1 | 0.045 | 0.206 | 4.56× |
| 100 | 5 | 0.210 | 1.094 | 5.20× |
| 100 | 10 | 0.418 | 2.183 | 5.22× |
| 1,000 | 1 | 0.057 | 0.660 | 11.57× |
| 1,000 | 5 | 0.196 | 3.313 | 16.93× |
| 1,000 | 10 | 0.401 | 6.771 | 16.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
