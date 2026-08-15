# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.26M | 0.012 | 85.67M | 0.039 | 2.56× | 3.31× |
| 10,000 | 0.139 | 71.77M | 0.143 | 69.69M | 0.128 | 0.92× | 0.89× |
| 100,000 | 1.437 | 69.59M | 1.339 | 74.69M | 1.012 | 0.70× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.181 | 1.54× |
| 1 | 5 | 0.244 | 0.478 | 1.96× |
| 1 | 10 | 0.398 | 0.955 | 2.40× |
| 10 | 1 | 0.042 | 0.093 | 2.24× |
| 10 | 5 | 0.189 | 0.433 | 2.29× |
| 10 | 10 | 0.446 | 0.933 | 2.09× |
| 100 | 1 | 0.045 | 0.090 | 2.01× |
| 100 | 5 | 0.194 | 0.438 | 2.25× |
| 100 | 10 | 0.431 | 0.939 | 2.18× |
| 1,000 | 1 | 0.060 | 0.104 | 1.74× |
| 1,000 | 5 | 0.188 | 0.481 | 2.55× |
| 1,000 | 10 | 0.437 | 1.024 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
