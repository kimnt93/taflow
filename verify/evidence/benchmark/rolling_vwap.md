# RollingVolumeWeightedAveragePrice benchmark (`RollingVWAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.48M | 0.023 | 43.07M | 0.282 | 10.57× | 12.15× |
| 10,000 | 0.230 | 43.48M | 0.228 | 43.85M | 1.406 | 6.11× | 6.17× |
| 100,000 | 2.265 | 44.16M | 2.150 | 46.52M | 13.030 | 5.75× | 6.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.281 | 3.31× |
| 1 | 5 | 0.217 | 1.386 | 6.38× |
| 1 | 10 | 0.419 | 2.613 | 6.24× |
| 10 | 1 | 0.047 | 0.205 | 4.39× |
| 10 | 5 | 0.195 | 1.002 | 5.14× |
| 10 | 10 | 0.421 | 2.561 | 6.09× |
| 100 | 1 | 0.051 | 0.217 | 4.24× |
| 100 | 5 | 0.217 | 1.105 | 5.09× |
| 100 | 10 | 0.563 | 2.515 | 4.47× |
| 1,000 | 1 | 0.072 | 0.341 | 4.76× |
| 1,000 | 5 | 0.220 | 1.723 | 7.84× |
| 1,000 | 10 | 0.456 | 3.684 | 8.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
