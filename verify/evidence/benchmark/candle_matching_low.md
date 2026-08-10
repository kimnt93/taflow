# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.82M | 0.015 | 67.40M | 0.033 | 1.79× | 2.24× |
| 10,000 | 0.108 | 92.62M | 0.102 | 98.31M | 0.090 | 0.83× | 0.88× |
| 100,000 | 1.029 | 97.15M | 1.023 | 97.76M | 0.731 | 0.71× | 0.71× |
| 1,000,000 | 11.607 | 86.15M | 10.640 | 93.98M | 6.691 | 0.58× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.135 | 1.20× |
| 1 | 5 | 0.296 | 0.451 | 1.52× |
| 1 | 10 | 0.510 | 0.942 | 1.85× |
| 10 | 1 | 0.059 | 0.101 | 1.71× |
| 10 | 5 | 0.292 | 0.454 | 1.56× |
| 10 | 10 | 0.510 | 0.880 | 1.72× |
| 100 | 1 | 0.052 | 0.093 | 1.78× |
| 100 | 5 | 0.256 | 0.480 | 1.87× |
| 100 | 10 | 0.572 | 0.921 | 1.61× |
| 1,000 | 1 | 0.062 | 0.096 | 1.55× |
| 1,000 | 5 | 0.262 | 0.465 | 1.77× |
| 1,000 | 10 | 0.680 | 1.131 | 1.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
