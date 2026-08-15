# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.77M | 0.013 | 77.43M | 0.037 | 2.33× | 2.88× |
| 10,000 | 0.138 | 72.54M | 0.131 | 76.57M | 0.114 | 0.83× | 0.87× |
| 100,000 | 1.342 | 74.53M | 1.325 | 75.50M | 0.829 | 0.62× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.125 | 0.95× |
| 1 | 5 | 0.220 | 0.470 | 2.14× |
| 1 | 10 | 0.396 | 1.031 | 2.61× |
| 10 | 1 | 0.047 | 0.098 | 2.08× |
| 10 | 5 | 0.201 | 0.458 | 2.28× |
| 10 | 10 | 0.378 | 0.970 | 2.57× |
| 100 | 1 | 0.052 | 0.096 | 1.86× |
| 100 | 5 | 0.196 | 0.490 | 2.50× |
| 100 | 10 | 0.375 | 0.964 | 2.57× |
| 1,000 | 1 | 0.057 | 0.102 | 1.78× |
| 1,000 | 5 | 0.197 | 0.532 | 2.69× |
| 1,000 | 10 | 0.453 | 1.020 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
