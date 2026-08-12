# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.13M | 0.032 | 30.92M | 0.226 | 6.58× | 6.99× |
| 10,000 | 0.296 | 33.74M | 0.287 | 34.86M | 1.014 | 3.42× | 3.53× |
| 100,000 | 2.925 | 34.19M | 2.876 | 34.77M | 9.012 | 3.08× | 3.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.262 | 3.40× |
| 1 | 5 | 0.325 | 1.088 | 3.35× |
| 1 | 10 | 0.469 | 2.320 | 4.95× |
| 10 | 1 | 0.055 | 0.215 | 3.91× |
| 10 | 5 | 0.230 | 1.237 | 5.38× |
| 10 | 10 | 0.504 | 2.354 | 4.67× |
| 100 | 1 | 0.061 | 0.233 | 3.83× |
| 100 | 5 | 0.244 | 1.305 | 5.34× |
| 100 | 10 | 0.528 | 2.388 | 4.52× |
| 1,000 | 1 | 0.083 | 0.305 | 3.68× |
| 1,000 | 5 | 0.261 | 1.813 | 6.95× |
| 1,000 | 10 | 0.519 | 3.214 | 6.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
