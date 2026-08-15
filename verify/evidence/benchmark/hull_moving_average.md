# HullMovingAverage benchmark (`HMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.90M | 0.024 | 41.93M | 0.160 | 6.85× | 6.70× |
| 10,000 | 0.229 | 43.69M | 0.214 | 46.67M | 0.537 | 2.35× | 2.51× |
| 100,000 | 2.050 | 48.78M | 2.331 | 42.91M | 4.372 | 2.13× | 1.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.220 | 2.75× |
| 1 | 5 | 0.257 | 0.982 | 3.82× |
| 1 | 10 | 0.424 | 2.139 | 5.04× |
| 10 | 1 | 0.044 | 0.187 | 4.24× |
| 10 | 5 | 0.190 | 0.925 | 4.87× |
| 10 | 10 | 0.421 | 2.087 | 4.95× |
| 100 | 1 | 0.051 | 0.196 | 3.86× |
| 100 | 5 | 0.199 | 0.998 | 5.01× |
| 100 | 10 | 0.418 | 2.233 | 5.34× |
| 1,000 | 1 | 0.073 | 0.254 | 3.50× |
| 1,000 | 5 | 0.226 | 1.212 | 5.37× |
| 1,000 | 10 | 0.430 | 2.595 | 6.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
