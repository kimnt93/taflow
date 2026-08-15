# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.16M | 0.018 | 54.05M | 0.620 | 29.23× | 33.51× |
| 10,000 | 0.165 | 60.48M | 0.166 | 60.25M | 4.094 | 24.76× | 24.67× |
| 100,000 | 1.572 | 63.63M | 1.604 | 62.35M | 44.937 | 28.59× | 28.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.326 | 4.04× |
| 1 | 5 | 0.288 | 1.436 | 4.99× |
| 1 | 10 | 0.399 | 3.021 | 7.57× |
| 10 | 1 | 0.049 | 0.273 | 5.53× |
| 10 | 5 | 0.191 | 1.521 | 7.94× |
| 10 | 10 | 0.428 | 3.105 | 7.26× |
| 100 | 1 | 0.052 | 0.341 | 6.53× |
| 100 | 5 | 0.216 | 1.688 | 7.83× |
| 100 | 10 | 0.423 | 3.443 | 8.14× |
| 1,000 | 1 | 0.064 | 0.758 | 11.93× |
| 1,000 | 5 | 0.224 | 3.704 | 16.51× |
| 1,000 | 10 | 0.463 | 7.588 | 16.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
