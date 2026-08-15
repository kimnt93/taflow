# CumulativeCount benchmark (`one-based cumulative count` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 574.03M | 0.001 | 1.24G | 0.012 | 7.14× | 15.49× |
| 10,000 | 0.005 | 2.07G | 0.003 | 3.93G | 0.017 | 3.52× | 6.70× |
| 100,000 | 0.045 | 2.24G | 0.021 | 4.67G | 0.059 | 1.31× | 2.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.077 | 1.42× |
| 1 | 5 | 0.215 | 0.294 | 1.37× |
| 1 | 10 | 0.391 | 0.583 | 1.49× |
| 10 | 1 | 0.041 | 0.060 | 1.46× |
| 10 | 5 | 0.177 | 0.279 | 1.58× |
| 10 | 10 | 0.368 | 0.604 | 1.64× |
| 100 | 1 | 0.049 | 0.064 | 1.31× |
| 100 | 5 | 0.177 | 0.305 | 1.73× |
| 100 | 10 | 0.377 | 0.618 | 1.64× |
| 1,000 | 1 | 0.040 | 0.058 | 1.46× |
| 1,000 | 5 | 0.186 | 0.283 | 1.52× |
| 1,000 | 10 | 0.413 | 0.614 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
