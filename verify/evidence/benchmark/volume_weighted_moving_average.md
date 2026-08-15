# VolumeWeightedMovingAverage benchmark (`VWMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.79M | 0.014 | 71.15M | 0.183 | 12.20× | 13.00× |
| 10,000 | 0.129 | 77.23M | 0.126 | 79.18M | 0.756 | 5.84× | 5.98× |
| 100,000 | 1.831 | 54.62M | 1.374 | 72.77M | 7.195 | 3.93× | 5.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.304 | 5.09× |
| 1 | 5 | 0.251 | 1.004 | 4.00× |
| 1 | 10 | 0.421 | 2.286 | 5.43× |
| 10 | 1 | 0.049 | 0.234 | 4.81× |
| 10 | 5 | 0.214 | 0.987 | 4.61× |
| 10 | 10 | 0.416 | 2.357 | 5.67× |
| 100 | 1 | 0.054 | 0.214 | 4.00× |
| 100 | 5 | 0.241 | 1.009 | 4.20× |
| 100 | 10 | 0.504 | 2.339 | 4.64× |
| 1,000 | 1 | 0.064 | 0.256 | 4.00× |
| 1,000 | 5 | 0.210 | 1.341 | 6.40× |
| 1,000 | 10 | 0.482 | 2.973 | 6.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
