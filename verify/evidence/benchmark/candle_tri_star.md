# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.24M | 0.014 | 70.89M | 0.033 | 1.85× | 2.37× |
| 10,000 | 0.102 | 98.12M | 0.102 | 97.74M | 0.086 | 0.85× | 0.84× |
| 100,000 | 1.000 | 99.95M | 0.977 | 102.34M | 0.591 | 0.59× | 0.60× |
| 1,000,000 | 10.150 | 98.52M | 10.744 | 93.08M | 6.300 | 0.62× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.123 | 1.45× |
| 1 | 5 | 0.261 | 0.489 | 1.87× |
| 1 | 10 | 0.538 | 0.899 | 1.67× |
| 10 | 1 | 0.053 | 0.086 | 1.63× |
| 10 | 5 | 0.249 | 0.435 | 1.75× |
| 10 | 10 | 0.538 | 0.924 | 1.72× |
| 100 | 1 | 0.057 | 0.088 | 1.54× |
| 100 | 5 | 0.250 | 0.432 | 1.73× |
| 100 | 10 | 0.531 | 0.920 | 1.73× |
| 1,000 | 1 | 0.067 | 0.098 | 1.45× |
| 1,000 | 5 | 0.257 | 0.460 | 1.79× |
| 1,000 | 10 | 0.735 | 1.237 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
