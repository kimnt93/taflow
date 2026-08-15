# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.76M | 0.003 | 390.07M | 0.033 | 5.83× | 12.86× |
| 10,000 | 0.056 | 178.74M | 0.050 | 201.24M | 0.133 | 2.37× | 2.67× |
| 100,000 | 0.632 | 158.10M | 0.605 | 165.30M | 1.095 | 1.73× | 1.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.124 | 1.63× |
| 1 | 5 | 0.336 | 0.477 | 1.42× |
| 1 | 10 | 0.380 | 0.869 | 2.29× |
| 10 | 1 | 0.043 | 0.090 | 2.11× |
| 10 | 5 | 0.181 | 0.401 | 2.21× |
| 10 | 10 | 0.364 | 0.871 | 2.40× |
| 100 | 1 | 0.040 | 0.089 | 2.20× |
| 100 | 5 | 0.180 | 0.417 | 2.32× |
| 100 | 10 | 0.380 | 0.882 | 2.32× |
| 1,000 | 1 | 0.055 | 0.100 | 1.82× |
| 1,000 | 5 | 0.185 | 0.495 | 2.67× |
| 1,000 | 10 | 0.434 | 1.015 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
