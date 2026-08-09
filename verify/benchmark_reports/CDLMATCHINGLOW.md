# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.09M | 0.007 | 147.04M | 0.035 | 4.06× | 5.14× |
| 10,000 | 0.053 | 189.82M | 0.049 | 202.83M | 0.091 | 1.73× | 1.85× |
| 100,000 | 0.532 | 187.99M | 0.510 | 196.04M | 0.661 | 1.24× | 1.30× |
| 1,000,000 | 5.666 | 176.48M | 5.456 | 183.28M | 6.084 | 1.07× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.161 | 1.43× |
| 1 | 5 | 0.330 | 0.530 | 1.61× |
| 1 | 10 | 0.554 | 0.950 | 1.72× |
| 10 | 1 | 0.056 | 0.090 | 1.62× |
| 10 | 5 | 0.240 | 0.436 | 1.82× |
| 10 | 10 | 0.489 | 0.941 | 1.92× |
| 100 | 1 | 0.055 | 0.100 | 1.82× |
| 100 | 5 | 0.253 | 0.453 | 1.79× |
| 100 | 10 | 0.512 | 0.925 | 1.81× |
| 1,000 | 1 | 0.061 | 0.100 | 1.62× |
| 1,000 | 5 | 0.259 | 0.484 | 1.87× |
| 1,000 | 10 | 0.562 | 0.998 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
