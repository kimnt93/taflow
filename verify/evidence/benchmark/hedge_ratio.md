# HedgeRatio benchmark (`rolling OLS hedge ratio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.86M | 0.039 | 25.61M | 0.255 | 6.33× | 6.52× |
| 10,000 | 0.388 | 25.75M | 0.391 | 25.55M | 1.581 | 4.07× | 4.04× |
| 100,000 | 4.033 | 24.80M | 4.049 | 24.70M | 18.265 | 4.53× | 4.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.169 | 2.41× |
| 1 | 5 | 0.282 | 0.734 | 2.61× |
| 1 | 10 | 0.409 | 1.247 | 3.05× |
| 10 | 1 | 0.045 | 0.120 | 2.68× |
| 10 | 5 | 0.184 | 0.623 | 3.38× |
| 10 | 10 | 0.415 | 1.243 | 3.00× |
| 100 | 1 | 0.046 | 0.210 | 4.53× |
| 100 | 5 | 0.198 | 1.089 | 5.49× |
| 100 | 10 | 0.463 | 2.301 | 4.97× |
| 1,000 | 1 | 0.094 | 0.356 | 3.81× |
| 1,000 | 5 | 0.192 | 1.469 | 7.64× |
| 1,000 | 10 | 0.489 | 2.883 | 5.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
