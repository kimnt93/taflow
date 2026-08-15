# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.55M | 0.004 | 264.12M | 0.034 | 4.93× | 9.07× |
| 10,000 | 0.100 | 100.10M | 0.098 | 101.54M | 0.130 | 1.31× | 1.32× |
| 100,000 | 1.034 | 96.70M | 1.020 | 98.01M | 0.994 | 0.96× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.105 | 1.79× |
| 1 | 5 | 0.227 | 0.464 | 2.05× |
| 1 | 10 | 0.411 | 0.910 | 2.22× |
| 10 | 1 | 0.045 | 0.089 | 1.96× |
| 10 | 5 | 0.183 | 0.435 | 2.38× |
| 10 | 10 | 0.386 | 0.895 | 2.32× |
| 100 | 1 | 0.052 | 0.097 | 1.84× |
| 100 | 5 | 0.190 | 0.433 | 2.28× |
| 100 | 10 | 0.400 | 0.888 | 2.22× |
| 1,000 | 1 | 0.055 | 0.099 | 1.80× |
| 1,000 | 5 | 0.200 | 0.485 | 2.43× |
| 1,000 | 10 | 0.411 | 0.993 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
