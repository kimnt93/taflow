# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.181 | 5.52M | 0.180 | 5.54M | 0.374 | 2.06× | 2.07× |
| 10,000 | 1.878 | 5.32M | 1.813 | 5.51M | 2.375 | 1.26× | 1.31× |
| 100,000 | 19.037 | 5.25M | 19.070 | 5.24M | 22.339 | 1.17× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.295 | 3.81× |
| 1 | 5 | 0.267 | 1.309 | 4.91× |
| 1 | 10 | 0.448 | 2.520 | 5.62× |
| 10 | 1 | 0.048 | 0.241 | 5.04× |
| 10 | 5 | 0.188 | 1.418 | 7.53× |
| 10 | 10 | 0.398 | 2.561 | 6.44× |
| 100 | 1 | 0.052 | 0.286 | 5.50× |
| 100 | 5 | 0.221 | 1.437 | 6.51× |
| 100 | 10 | 0.422 | 2.622 | 6.22× |
| 1,000 | 1 | 0.240 | 0.458 | 1.91× |
| 1,000 | 5 | 0.348 | 2.455 | 7.06× |
| 1,000 | 10 | 0.585 | 4.942 | 8.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
