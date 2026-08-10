# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.87M | 0.017 | 58.78M | 0.035 | 1.75× | 2.06× |
| 10,000 | 0.140 | 71.31M | 0.138 | 72.63M | 0.126 | 0.90× | 0.92× |
| 100,000 | 1.339 | 74.70M | 1.577 | 63.40M | 0.975 | 0.73× | 0.62× |
| 1,000,000 | 14.100 | 70.92M | 14.440 | 69.25M | 10.775 | 0.76× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.107 | 0.88× |
| 1 | 5 | 0.317 | 0.450 | 1.42× |
| 1 | 10 | 0.516 | 0.893 | 1.73× |
| 10 | 1 | 0.054 | 0.094 | 1.74× |
| 10 | 5 | 0.238 | 0.418 | 1.75× |
| 10 | 10 | 0.522 | 0.873 | 1.67× |
| 100 | 1 | 0.066 | 0.090 | 1.37× |
| 100 | 5 | 0.262 | 0.425 | 1.62× |
| 100 | 10 | 0.500 | 0.916 | 1.83× |
| 1,000 | 1 | 0.068 | 0.101 | 1.48× |
| 1,000 | 5 | 0.262 | 0.473 | 1.81× |
| 1,000 | 10 | 0.547 | 1.031 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
