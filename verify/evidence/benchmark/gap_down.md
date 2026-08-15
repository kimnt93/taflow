# GapDown benchmark (`gap down relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.68M | 0.004 | 257.19M | 0.024 | 4.71× | 6.06× |
| 10,000 | 0.032 | 315.01M | 0.028 | 357.31M | 0.042 | 1.33× | 1.51× |
| 100,000 | 0.267 | 374.59M | 0.242 | 413.83M | 0.247 | 0.92× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.134 | 1.61× |
| 1 | 5 | 0.227 | 0.365 | 1.61× |
| 1 | 10 | 0.398 | 0.715 | 1.80× |
| 10 | 1 | 0.041 | 0.070 | 1.69× |
| 10 | 5 | 0.183 | 0.337 | 1.85× |
| 10 | 10 | 0.375 | 0.753 | 2.01× |
| 100 | 1 | 0.044 | 0.073 | 1.65× |
| 100 | 5 | 0.181 | 0.344 | 1.90× |
| 100 | 10 | 0.413 | 0.765 | 1.85× |
| 1,000 | 1 | 0.056 | 0.097 | 1.71× |
| 1,000 | 5 | 0.204 | 0.534 | 2.62× |
| 1,000 | 10 | 0.439 | 1.174 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
