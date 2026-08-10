# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.48M | 0.017 | 60.18M | 0.041 | 2.05× | 2.44× |
| 10,000 | 0.155 | 64.58M | 0.137 | 72.99M | 0.121 | 0.78× | 0.89× |
| 100,000 | 1.405 | 71.19M | 1.387 | 72.07M | 0.933 | 0.66× | 0.67× |
| 1,000,000 | 14.448 | 69.21M | 14.715 | 67.96M | 9.460 | 0.65× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.153 | 1.98× |
| 1 | 5 | 0.320 | 0.529 | 1.66× |
| 1 | 10 | 0.565 | 1.107 | 1.96× |
| 10 | 1 | 0.064 | 0.109 | 1.71× |
| 10 | 5 | 0.281 | 0.484 | 1.72× |
| 10 | 10 | 0.548 | 0.985 | 1.80× |
| 100 | 1 | 0.066 | 0.112 | 1.70× |
| 100 | 5 | 0.345 | 0.577 | 1.67× |
| 100 | 10 | 0.639 | 0.999 | 1.56× |
| 1,000 | 1 | 0.068 | 0.140 | 2.05× |
| 1,000 | 5 | 0.317 | 0.621 | 1.96× |
| 1,000 | 10 | 0.613 | 1.079 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
