# YangZhang benchmark (`YangZhangVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.26M | 0.049 | 20.51M | 0.314 | 5.74× | 6.44× |
| 10,000 | 0.450 | 22.21M | 0.436 | 22.94M | 1.884 | 4.18× | 4.32× |
| 100,000 | 4.507 | 22.19M | 4.512 | 22.17M | 16.856 | 3.74× | 3.74× |
| 1,000,000 | 44.059 | 22.70M | 42.697 | 23.42M | 157.995 | 3.59× | 3.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.298 | 2.85× |
| 1 | 5 | 0.376 | 1.434 | 3.81× |
| 1 | 10 | 0.510 | 2.737 | 5.37× |
| 10 | 1 | 0.060 | 0.247 | 4.09× |
| 10 | 5 | 0.257 | 1.489 | 5.80× |
| 10 | 10 | 0.513 | 2.526 | 4.93× |
| 100 | 1 | 0.061 | 0.258 | 4.25× |
| 100 | 5 | 0.253 | 1.515 | 6.00× |
| 100 | 10 | 0.569 | 2.886 | 5.07× |
| 1,000 | 1 | 0.105 | 0.405 | 3.87× |
| 1,000 | 5 | 0.275 | 2.340 | 8.51× |
| 1,000 | 10 | 0.689 | 4.442 | 6.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
