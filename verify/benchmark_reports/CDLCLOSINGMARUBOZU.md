# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.79M | 0.008 | 120.77M | 0.037 | 3.66× | 4.43× |
| 10,000 | 0.099 | 100.59M | 0.094 | 106.15M | 0.131 | 1.31× | 1.39× |
| 100,000 | 0.957 | 104.48M | 0.947 | 105.55M | 1.018 | 1.06× | 1.07× |
| 1,000,000 | 9.832 | 101.71M | 9.638 | 103.76M | 10.030 | 1.02× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.116 | 1.32× |
| 1 | 5 | 0.304 | 0.497 | 1.64× |
| 1 | 10 | 0.484 | 0.886 | 1.83× |
| 10 | 1 | 0.052 | 0.089 | 1.69× |
| 10 | 5 | 0.240 | 0.447 | 1.86× |
| 10 | 10 | 0.515 | 0.918 | 1.78× |
| 100 | 1 | 0.056 | 0.094 | 1.69× |
| 100 | 5 | 0.268 | 0.457 | 1.71× |
| 100 | 10 | 0.502 | 0.913 | 1.82× |
| 1,000 | 1 | 0.075 | 0.114 | 1.53× |
| 1,000 | 5 | 0.264 | 0.508 | 1.93× |
| 1,000 | 10 | 0.529 | 1.063 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
