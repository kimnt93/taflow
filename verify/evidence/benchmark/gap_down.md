# GapDown benchmark (`gap down relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 194.51M | 0.004 | 276.71M | 0.023 | 4.54× | 6.46× |
| 10,000 | 0.030 | 330.76M | 0.027 | 369.22M | 0.048 | 1.59× | 1.77× |
| 100,000 | 0.293 | 341.68M | 0.252 | 396.75M | 0.238 | 0.81× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.124 | 1.72× |
| 1 | 5 | 0.310 | 0.366 | 1.18× |
| 1 | 10 | 0.408 | 0.787 | 1.93× |
| 10 | 1 | 0.042 | 0.082 | 1.95× |
| 10 | 5 | 0.184 | 0.349 | 1.89× |
| 10 | 10 | 0.390 | 0.769 | 1.97× |
| 100 | 1 | 0.045 | 0.073 | 1.63× |
| 100 | 5 | 0.197 | 0.384 | 1.95× |
| 100 | 10 | 0.444 | 0.790 | 1.78× |
| 1,000 | 1 | 0.045 | 0.085 | 1.87× |
| 1,000 | 5 | 0.200 | 0.491 | 2.46× |
| 1,000 | 10 | 0.408 | 13.997 | 34.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
