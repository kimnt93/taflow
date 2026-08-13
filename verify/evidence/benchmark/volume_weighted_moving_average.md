# VolumeWeightedMovingAverage benchmark (`VWMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.285 | 3.51M | 0.279 | 3.59M | 0.179 | 0.63× | 0.64× |
| 10,000 | 2.725 | 3.67M | 2.753 | 3.63M | 0.747 | 0.27× | 0.27× |
| 100,000 | 27.511 | 3.63M | 27.388 | 3.65M | 6.466 | 0.24× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.270 | 2.39× |
| 1 | 5 | 0.413 | 0.976 | 2.37× |
| 1 | 10 | 0.641 | 2.099 | 3.28× |
| 10 | 1 | 0.076 | 0.196 | 2.57× |
| 10 | 5 | 0.315 | 0.948 | 3.01× |
| 10 | 10 | 0.643 | 2.152 | 3.35× |
| 100 | 1 | 0.102 | 0.207 | 2.03× |
| 100 | 5 | 0.319 | 0.967 | 3.03× |
| 100 | 10 | 0.666 | 2.162 | 3.25× |
| 1,000 | 1 | 0.349 | 0.264 | 0.76× |
| 1,000 | 5 | 0.619 | 1.282 | 2.07× |
| 1,000 | 10 | 0.983 | 2.775 | 2.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
