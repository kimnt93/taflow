# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.28M | 0.008 | 123.40M | 0.036 | 3.31× | 4.48× |
| 10,000 | 0.123 | 81.31M | 0.114 | 87.59M | 0.109 | 0.89× | 0.96× |
| 100,000 | 1.217 | 82.20M | 1.184 | 84.48M | 0.854 | 0.70× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.160 | 1.67× |
| 1 | 5 | 0.351 | 0.490 | 1.40× |
| 1 | 10 | 0.403 | 0.951 | 2.36× |
| 10 | 1 | 0.042 | 0.093 | 2.21× |
| 10 | 5 | 0.212 | 0.487 | 2.30× |
| 10 | 10 | 0.391 | 0.960 | 2.45× |
| 100 | 1 | 0.042 | 0.100 | 2.37× |
| 100 | 5 | 0.190 | 0.451 | 2.38× |
| 100 | 10 | 0.437 | 0.978 | 2.24× |
| 1,000 | 1 | 0.068 | 0.101 | 1.48× |
| 1,000 | 5 | 0.198 | 0.494 | 2.50× |
| 1,000 | 10 | 0.417 | 1.112 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
