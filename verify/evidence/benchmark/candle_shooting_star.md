# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.02M | 0.016 | 60.67M | 0.045 | 2.39× | 2.73× |
| 10,000 | 0.166 | 60.26M | 0.157 | 63.65M | 0.180 | 1.08× | 1.14× |
| 100,000 | 1.638 | 61.06M | 1.624 | 61.56M | 1.475 | 0.90× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.091 | 1.07× |
| 1 | 5 | 0.411 | 0.446 | 1.09× |
| 1 | 10 | 0.543 | 0.985 | 1.82× |
| 10 | 1 | 0.059 | 0.090 | 1.51× |
| 10 | 5 | 0.259 | 0.441 | 1.70× |
| 10 | 10 | 0.546 | 0.950 | 1.74× |
| 100 | 1 | 0.076 | 0.099 | 1.30× |
| 100 | 5 | 0.282 | 0.497 | 1.76× |
| 100 | 10 | 0.550 | 0.949 | 1.73× |
| 1,000 | 1 | 0.081 | 0.099 | 1.22× |
| 1,000 | 5 | 0.312 | 0.560 | 1.80× |
| 1,000 | 10 | 0.547 | 1.100 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
