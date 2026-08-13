# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.79M | 0.036 | 28.13M | 0.034 | 0.89× | 0.97× |
| 10,000 | 0.246 | 40.71M | 0.233 | 43.01M | 0.058 | 0.24× | 0.25× |
| 100,000 | 2.712 | 36.87M | 2.345 | 42.64M | 0.296 | 0.11× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.136 | 1.09× |
| 1 | 5 | 0.378 | 0.478 | 1.26× |
| 1 | 10 | 0.619 | 0.971 | 1.57× |
| 10 | 1 | 0.064 | 0.092 | 1.43× |
| 10 | 5 | 0.304 | 0.449 | 1.48× |
| 10 | 10 | 0.631 | 0.968 | 1.53× |
| 100 | 1 | 0.068 | 0.095 | 1.40× |
| 100 | 5 | 0.300 | 0.445 | 1.48× |
| 100 | 10 | 0.652 | 0.956 | 1.47× |
| 1,000 | 1 | 0.095 | 0.100 | 1.06× |
| 1,000 | 5 | 0.313 | 0.463 | 1.48× |
| 1,000 | 10 | 0.655 | 0.995 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
