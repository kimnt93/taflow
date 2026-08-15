# RollingZScore benchmark (`ZScore` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 31.81M | 0.032 | 31.12M | 0.196 | 6.25× | 6.11× |
| 10,000 | 0.291 | 34.32M | 0.276 | 36.25M | 0.550 | 1.89× | 1.99× |
| 100,000 | 2.871 | 34.83M | 2.911 | 34.36M | 4.567 | 1.59× | 1.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.243 | 4.53× |
| 1 | 5 | 0.242 | 1.224 | 5.06× |
| 1 | 10 | 0.387 | 2.282 | 5.89× |
| 10 | 1 | 0.044 | 0.209 | 4.71× |
| 10 | 5 | 0.190 | 1.310 | 6.90× |
| 10 | 10 | 0.412 | 2.311 | 5.61× |
| 100 | 1 | 0.049 | 0.213 | 4.36× |
| 100 | 5 | 0.196 | 1.226 | 6.25× |
| 100 | 10 | 0.436 | 2.373 | 5.44× |
| 1,000 | 1 | 0.084 | 0.260 | 3.11× |
| 1,000 | 5 | 0.215 | 1.414 | 6.57× |
| 1,000 | 10 | 0.504 | 2.661 | 5.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
