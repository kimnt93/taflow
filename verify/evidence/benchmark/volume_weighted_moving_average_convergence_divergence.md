# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.28M | 0.018 | 55.59M | 0.606 | 29.86× | 33.68× |
| 10,000 | 0.156 | 64.17M | 0.149 | 66.95M | 4.064 | 26.08× | 27.21× |
| 100,000 | 1.658 | 60.31M | 1.438 | 69.53M | 45.251 | 27.29× | 31.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.311 | 3.92× |
| 1 | 5 | 0.339 | 1.715 | 5.06× |
| 1 | 10 | 0.414 | 3.077 | 7.43× |
| 10 | 1 | 0.060 | 0.307 | 5.08× |
| 10 | 5 | 0.199 | 1.490 | 7.49× |
| 10 | 10 | 0.420 | 3.154 | 7.51× |
| 100 | 1 | 0.052 | 0.318 | 6.11× |
| 100 | 5 | 0.198 | 1.666 | 8.42× |
| 100 | 10 | 0.472 | 3.410 | 7.22× |
| 1,000 | 1 | 0.066 | 0.808 | 12.30× |
| 1,000 | 5 | 0.243 | 3.844 | 15.79× |
| 1,000 | 10 | 0.498 | 7.638 | 15.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
