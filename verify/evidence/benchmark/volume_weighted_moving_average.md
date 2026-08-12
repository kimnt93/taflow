# VolumeWeightedMovingAverage benchmark (`VWMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.32M | 0.016 | 62.12M | 0.195 | 10.99× | 12.13× |
| 10,000 | 0.132 | 75.55M | 0.128 | 77.87M | 0.803 | 6.06× | 6.25× |
| 100,000 | 1.266 | 78.97M | 1.230 | 81.28M | 6.796 | 5.37× | 5.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.267 | 1.60× |
| 1 | 5 | 0.282 | 1.001 | 3.55× |
| 1 | 10 | 0.486 | 2.180 | 4.49× |
| 10 | 1 | 0.051 | 0.211 | 4.12× |
| 10 | 5 | 0.235 | 0.984 | 4.19× |
| 10 | 10 | 0.494 | 2.249 | 4.55× |
| 100 | 1 | 0.053 | 0.203 | 3.82× |
| 100 | 5 | 0.252 | 1.074 | 4.26× |
| 100 | 10 | 0.540 | 2.377 | 4.40× |
| 1,000 | 1 | 0.068 | 0.261 | 3.83× |
| 1,000 | 5 | 0.250 | 1.343 | 5.38× |
| 1,000 | 10 | 0.560 | 2.863 | 5.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
