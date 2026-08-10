# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.42M | 0.014 | 70.47M | 0.045 | 3.08× | 3.17× |
| 10,000 | 0.154 | 64.96M | 0.165 | 60.51M | 0.198 | 1.29× | 1.20× |
| 100,000 | 1.630 | 61.34M | 1.604 | 62.33M | 1.559 | 0.96× | 0.97× |
| 1,000,000 | 15.787 | 63.34M | 16.650 | 60.06M | 16.409 | 1.04× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.130 | 1.43× |
| 1 | 5 | 0.244 | 0.446 | 1.83× |
| 1 | 10 | 0.457 | 0.982 | 2.15× |
| 10 | 1 | 0.054 | 0.088 | 1.62× |
| 10 | 5 | 0.249 | 0.474 | 1.91× |
| 10 | 10 | 0.505 | 0.902 | 1.79× |
| 100 | 1 | 0.048 | 0.103 | 2.13× |
| 100 | 5 | 0.254 | 0.439 | 1.73× |
| 100 | 10 | 0.494 | 0.919 | 1.86× |
| 1,000 | 1 | 0.070 | 0.111 | 1.59× |
| 1,000 | 5 | 0.260 | 0.573 | 2.20× |
| 1,000 | 10 | 0.550 | 1.139 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
