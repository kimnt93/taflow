# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.50M | 0.018 | 56.41M | 0.037 | 2.00× | 2.07× |
| 10,000 | 0.149 | 66.94M | 0.166 | 60.23M | 0.115 | 0.77× | 0.69× |
| 100,000 | 1.492 | 67.03M | 1.631 | 61.32M | 0.888 | 0.60× | 0.54× |
| 1,000,000 | 16.100 | 62.11M | 17.306 | 57.78M | 8.360 | 0.52× | 0.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.126 | 1.31× |
| 1 | 5 | 0.399 | 0.495 | 1.24× |
| 1 | 10 | 0.537 | 0.951 | 1.77× |
| 10 | 1 | 0.053 | 0.095 | 1.78× |
| 10 | 5 | 0.244 | 0.451 | 1.85× |
| 10 | 10 | 0.525 | 0.960 | 1.83× |
| 100 | 1 | 0.058 | 0.093 | 1.61× |
| 100 | 5 | 0.256 | 0.465 | 1.82× |
| 100 | 10 | 0.543 | 0.969 | 1.78× |
| 1,000 | 1 | 0.072 | 0.101 | 1.40× |
| 1,000 | 5 | 0.282 | 0.526 | 1.87× |
| 1,000 | 10 | 0.571 | 1.059 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
