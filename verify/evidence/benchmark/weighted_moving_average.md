# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.71M | 0.005 | 186.35M | 0.035 | 5.64× | 6.46× |
| 10,000 | 0.036 | 278.83M | 0.033 | 301.81M | 0.049 | 1.36× | 1.48× |
| 100,000 | 0.326 | 306.97M | 0.299 | 334.98M | 0.209 | 0.64× | 0.70× |
| 1,000,000 | 3.853 | 259.52M | 2.998 | 333.61M | 2.101 | 0.55× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.101 | 1.56× |
| 1 | 5 | 0.257 | 0.498 | 1.93× |
| 1 | 10 | 0.494 | 1.008 | 2.04× |
| 10 | 1 | 0.050 | 0.093 | 1.85× |
| 10 | 5 | 0.219 | 0.488 | 2.23× |
| 10 | 10 | 0.517 | 0.990 | 1.92× |
| 100 | 1 | 0.049 | 0.095 | 1.93× |
| 100 | 5 | 0.215 | 0.436 | 2.03× |
| 100 | 10 | 0.465 | 0.899 | 1.94× |
| 1,000 | 1 | 0.055 | 0.091 | 1.65× |
| 1,000 | 5 | 0.225 | 0.471 | 2.10× |
| 1,000 | 10 | 0.491 | 0.916 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
