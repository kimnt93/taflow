# RollingVolumeWeightedAveragePrice benchmark (`RollingVWAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.12M | 0.023 | 43.39M | 0.257 | 10.03× | 11.13× |
| 10,000 | 0.238 | 42.09M | 0.226 | 44.24M | 1.366 | 5.75× | 6.04× |
| 100,000 | 2.128 | 46.99M | 2.095 | 47.72M | 12.039 | 5.66× | 5.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.313 | 4.17× |
| 1 | 5 | 0.206 | 1.442 | 6.99× |
| 1 | 10 | 0.421 | 2.270 | 5.40× |
| 10 | 1 | 0.045 | 0.203 | 4.54× |
| 10 | 5 | 0.189 | 1.015 | 5.38× |
| 10 | 10 | 0.406 | 2.295 | 5.66× |
| 100 | 1 | 0.047 | 0.206 | 4.41× |
| 100 | 5 | 0.199 | 1.027 | 5.17× |
| 100 | 10 | 0.434 | 2.381 | 5.48× |
| 1,000 | 1 | 0.067 | 0.321 | 4.77× |
| 1,000 | 5 | 0.199 | 1.604 | 8.07× |
| 1,000 | 10 | 0.436 | 3.578 | 8.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
