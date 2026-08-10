# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.53M | 0.011 | 89.73M | 0.373 | 25.16× | 33.44× |
| 10,000 | 0.054 | 184.04M | 0.045 | 220.89M | 2.436 | 44.83× | 53.81× |
| 100,000 | 0.392 | 254.89M | 0.376 | 266.17M | 21.121 | 53.83× | 56.22× |
| 1,000,000 | 4.523 | 221.07M | 4.139 | 241.58M | 245.715 | 54.32× | 59.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.532 | 5.92× |
| 1 | 5 | 0.364 | 1.173 | 3.22× |
| 1 | 10 | 0.594 | 2.625 | 4.42× |
| 10 | 1 | 0.075 | 0.236 | 3.15× |
| 10 | 5 | 0.285 | 1.195 | 4.19× |
| 10 | 10 | 0.618 | 2.528 | 4.09× |
| 100 | 1 | 0.067 | 0.257 | 3.82× |
| 100 | 5 | 0.286 | 1.445 | 5.05× |
| 100 | 10 | 0.606 | 2.680 | 4.43× |
| 1,000 | 1 | 0.063 | 0.471 | 7.46× |
| 1,000 | 5 | 0.298 | 2.530 | 8.49× |
| 1,000 | 10 | 0.685 | 5.531 | 8.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
