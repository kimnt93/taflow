# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.13M | 0.019 | 52.72M | 0.035 | 1.63× | 1.83× |
| 10,000 | 0.154 | 64.73M | 0.144 | 69.24M | 0.131 | 0.85× | 0.91× |
| 100,000 | 1.435 | 69.67M | 1.395 | 71.71M | 1.078 | 0.75× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.100 | 1.02× |
| 1 | 5 | 0.370 | 0.465 | 1.25× |
| 1 | 10 | 0.588 | 0.948 | 1.61× |
| 10 | 1 | 0.051 | 0.087 | 1.70× |
| 10 | 5 | 0.250 | 0.441 | 1.76× |
| 10 | 10 | 0.555 | 1.034 | 1.86× |
| 100 | 1 | 0.056 | 0.093 | 1.66× |
| 100 | 5 | 0.280 | 0.454 | 1.62× |
| 100 | 10 | 0.577 | 0.995 | 1.72× |
| 1,000 | 1 | 0.075 | 0.103 | 1.36× |
| 1,000 | 5 | 0.295 | 0.534 | 1.81× |
| 1,000 | 10 | 0.622 | 1.091 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
