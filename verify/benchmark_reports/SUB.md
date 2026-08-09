# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.88M | 0.003 | 298.76M | 0.032 | 6.08× | 9.47× |
| 10,000 | 0.011 | 879.81M | 0.007 | 1.35G | 0.036 | 3.20× | 4.91× |
| 100,000 | 0.071 | 1.41G | 0.044 | 2.26G | 0.075 | 1.06× | 1.70× |
| 1,000,000 | 1.252 | 799.03M | 0.893 | 1.12G | 0.853 | 0.68× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.121 | 1.01× |
| 1 | 5 | 0.313 | 0.559 | 1.78× |
| 1 | 10 | 0.527 | 0.991 | 1.88× |
| 10 | 1 | 0.056 | 0.090 | 1.62× |
| 10 | 5 | 0.265 | 0.484 | 1.82× |
| 10 | 10 | 0.523 | 0.971 | 1.86× |
| 100 | 1 | 0.052 | 0.093 | 1.77× |
| 100 | 5 | 0.261 | 0.498 | 1.91× |
| 100 | 10 | 0.568 | 1.073 | 1.89× |
| 1,000 | 1 | 0.059 | 0.105 | 1.78× |
| 1,000 | 5 | 0.236 | 0.475 | 2.01× |
| 1,000 | 10 | 0.493 | 0.914 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
