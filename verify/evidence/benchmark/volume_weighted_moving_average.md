# VolumeWeightedMovingAverage benchmark (`VWMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.03M | 0.015 | 66.84M | 0.184 | 11.26× | 12.33× |
| 10,000 | 0.121 | 82.95M | 0.124 | 80.65M | 0.764 | 6.34× | 6.17× |
| 100,000 | 1.256 | 79.63M | 1.280 | 78.10M | 6.323 | 5.03× | 4.94× |
| 1,000,000 | 12.748 | 78.45M | 20.892 | 47.87M | 63.169 | 4.96× | 3.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.242 | 2.72× |
| 1 | 5 | 0.287 | 0.955 | 3.33× |
| 1 | 10 | 0.508 | 2.077 | 4.09× |
| 10 | 1 | 0.051 | 0.192 | 3.74× |
| 10 | 5 | 0.241 | 0.948 | 3.94× |
| 10 | 10 | 0.485 | 2.193 | 4.52× |
| 100 | 1 | 0.055 | 0.199 | 3.61× |
| 100 | 5 | 0.248 | 0.987 | 3.98× |
| 100 | 10 | 0.513 | 2.151 | 4.20× |
| 1,000 | 1 | 0.068 | 0.265 | 3.88× |
| 1,000 | 5 | 0.243 | 1.364 | 5.61× |
| 1,000 | 10 | 0.546 | 2.818 | 5.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
