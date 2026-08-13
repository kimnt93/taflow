# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.83M | 0.120 | 8.35M | 0.593 | 4.64× | 4.95× |
| 10,000 | 1.121 | 8.92M | 1.125 | 8.89M | 4.033 | 3.60× | 3.58× |
| 100,000 | 11.055 | 9.05M | 10.863 | 9.21M | 44.095 | 3.99× | 4.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.354 | 3.40× |
| 1 | 5 | 0.409 | 1.527 | 3.73× |
| 1 | 10 | 0.694 | 3.027 | 4.36× |
| 10 | 1 | 0.076 | 0.287 | 3.80× |
| 10 | 5 | 0.331 | 1.496 | 4.53× |
| 10 | 10 | 0.683 | 3.132 | 4.59× |
| 100 | 1 | 0.093 | 0.326 | 3.48× |
| 100 | 5 | 0.327 | 1.674 | 5.11× |
| 100 | 10 | 0.711 | 3.469 | 4.88× |
| 1,000 | 1 | 0.193 | 0.784 | 4.06× |
| 1,000 | 5 | 0.441 | 3.735 | 8.47× |
| 1,000 | 10 | 0.746 | 7.868 | 10.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
