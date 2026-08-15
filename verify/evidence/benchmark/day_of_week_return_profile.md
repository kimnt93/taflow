# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.77M | 0.034 | 29.65M | 0.889 | 22.02× | 26.36× |
| 10,000 | 0.343 | 29.19M | 0.348 | 28.71M | 7.279 | 21.25× | 20.89× |
| 100,000 | 3.567 | 28.03M | 3.290 | 30.40M | 79.783 | 22.37× | 24.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.301 | 3.85× |
| 1 | 5 | 0.320 | 1.136 | 3.55× |
| 1 | 10 | 0.526 | 2.400 | 4.56× |
| 10 | 1 | 0.046 | 0.227 | 4.91× |
| 10 | 5 | 0.193 | 1.275 | 6.62× |
| 10 | 10 | 0.425 | 2.517 | 5.93× |
| 100 | 1 | 0.054 | 0.307 | 5.70× |
| 100 | 5 | 0.202 | 1.702 | 8.43× |
| 100 | 10 | 0.456 | 3.145 | 6.90× |
| 1,000 | 1 | 0.085 | 1.205 | 14.12× |
| 1,000 | 5 | 0.213 | 5.247 | 24.58× |
| 1,000 | 10 | 0.454 | 10.793 | 23.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
