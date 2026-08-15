# FibonacciChannel benchmark (`FibChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.84M | 0.016 | 62.23M | 0.528 | 27.35× | 32.84× |
| 10,000 | 0.168 | 59.52M | 0.167 | 59.96M | 4.219 | 25.11× | 25.30× |
| 100,000 | 1.780 | 56.18M | 1.544 | 64.75M | 51.830 | 29.12× | 33.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.246 | 3.04× |
| 1 | 5 | 0.363 | 0.902 | 2.48× |
| 1 | 10 | 0.445 | 2.013 | 4.53× |
| 10 | 1 | 0.053 | 0.188 | 3.56× |
| 10 | 5 | 0.250 | 0.960 | 3.83× |
| 10 | 10 | 0.417 | 2.067 | 4.95× |
| 100 | 1 | 0.054 | 0.261 | 4.85× |
| 100 | 5 | 0.224 | 1.054 | 4.70× |
| 100 | 10 | 0.465 | 2.434 | 5.24× |
| 1,000 | 1 | 0.070 | 0.799 | 11.37× |
| 1,000 | 5 | 0.205 | 3.378 | 16.44× |
| 1,000 | 10 | 0.463 | 7.110 | 15.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
