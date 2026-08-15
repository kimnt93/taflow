# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.10M | 0.003 | 303.31M | 0.041 | 6.46× | 12.47× |
| 10,000 | 0.091 | 110.45M | 0.085 | 118.27M | 0.158 | 1.74× | 1.87× |
| 100,000 | 1.006 | 99.36M | 1.007 | 99.33M | 1.366 | 1.36× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.101 | 1.44× |
| 1 | 5 | 0.259 | 0.450 | 1.74× |
| 1 | 10 | 0.376 | 0.904 | 2.40× |
| 10 | 1 | 0.040 | 0.090 | 2.23× |
| 10 | 5 | 0.178 | 0.418 | 2.35× |
| 10 | 10 | 0.366 | 0.892 | 2.44× |
| 100 | 1 | 0.042 | 0.088 | 2.08× |
| 100 | 5 | 0.181 | 0.448 | 2.48× |
| 100 | 10 | 0.401 | 0.979 | 2.44× |
| 1,000 | 1 | 0.051 | 0.106 | 2.07× |
| 1,000 | 5 | 0.177 | 0.499 | 2.82× |
| 1,000 | 10 | 0.390 | 1.066 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
