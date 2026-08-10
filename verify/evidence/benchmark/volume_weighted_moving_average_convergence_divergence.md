# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.11M | 0.022 | 45.15M | 0.631 | 25.92× | 28.47× |
| 10,000 | 0.195 | 51.39M | 0.186 | 53.79M | 4.259 | 21.89× | 22.91× |
| 100,000 | 1.857 | 53.84M | 1.753 | 57.04M | 46.164 | 24.85× | 26.33× |
| 1,000,000 | 20.057 | 49.86M | 18.708 | 53.45M | 484.662 | 24.16× | 25.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.340 | 2.91× |
| 1 | 5 | 0.338 | 1.602 | 4.74× |
| 1 | 10 | 0.502 | 3.152 | 6.29× |
| 10 | 1 | 0.070 | 0.325 | 4.62× |
| 10 | 5 | 0.252 | 1.542 | 6.11× |
| 10 | 10 | 0.501 | 3.456 | 6.90× |
| 100 | 1 | 0.055 | 0.324 | 5.91× |
| 100 | 5 | 0.242 | 1.761 | 7.27× |
| 100 | 10 | 0.529 | 3.551 | 6.72× |
| 1,000 | 1 | 0.085 | 0.873 | 10.31× |
| 1,000 | 5 | 0.263 | 3.858 | 14.69× |
| 1,000 | 10 | 0.605 | 7.615 | 12.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
