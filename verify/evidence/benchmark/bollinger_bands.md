# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.18M | 0.007 | 149.54M | 0.058 | 6.80× | 8.61× |
| 10,000 | 0.052 | 190.86M | 0.040 | 252.94M | 0.100 | 1.92× | 2.54× |
| 100,000 | 0.443 | 225.65M | 0.355 | 281.31M | 0.573 | 1.29× | 1.61× |
| 1,000,000 | 16.366 | 61.10M | 4.065 | 246.03M | 6.246 | 0.38× | 1.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.166 | 2.04× |
| 1 | 5 | 0.366 | 0.657 | 1.79× |
| 1 | 10 | 0.487 | 1.204 | 2.47× |
| 10 | 1 | 0.054 | 0.109 | 2.03× |
| 10 | 5 | 0.262 | 0.600 | 2.29× |
| 10 | 10 | 0.492 | 1.177 | 2.39× |
| 100 | 1 | 0.054 | 0.123 | 2.28× |
| 100 | 5 | 0.624 | 0.602 | 0.96× |
| 100 | 10 | 0.513 | 1.139 | 2.22× |
| 1,000 | 1 | 0.107 | 0.120 | 1.12× |
| 1,000 | 5 | 0.270 | 0.641 | 2.37× |
| 1,000 | 10 | 0.545 | 1.270 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
