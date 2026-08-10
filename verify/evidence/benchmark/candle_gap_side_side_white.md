# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.90M | 0.017 | 59.64M | 0.050 | 2.14× | 2.98× |
| 10,000 | 0.143 | 69.77M | 0.137 | 72.78M | 0.239 | 1.67× | 1.74× |
| 100,000 | 1.360 | 73.53M | 1.370 | 72.99M | 2.188 | 1.61× | 1.60× |
| 1,000,000 | 14.921 | 67.02M | 14.457 | 69.17M | 20.805 | 1.39× | 1.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.140 | 1.63× |
| 1 | 5 | 0.329 | 0.470 | 1.43× |
| 1 | 10 | 0.562 | 1.102 | 1.96× |
| 10 | 1 | 0.072 | 0.096 | 1.34× |
| 10 | 5 | 0.262 | 0.468 | 1.79× |
| 10 | 10 | 0.535 | 1.065 | 1.99× |
| 100 | 1 | 0.068 | 0.110 | 1.61× |
| 100 | 5 | 0.295 | 0.489 | 1.66× |
| 100 | 10 | 0.584 | 1.035 | 1.77× |
| 1,000 | 1 | 0.090 | 0.124 | 1.38× |
| 1,000 | 5 | 0.371 | 0.699 | 1.88× |
| 1,000 | 10 | 0.653 | 1.221 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
