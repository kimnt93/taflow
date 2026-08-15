# RollingCoefficientOfDetermination benchmark (`rolling squared correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.32M | 0.050 | 19.82M | 0.297 | 6.04× | 5.89× |
| 10,000 | 0.476 | 21.03M | 0.479 | 20.87M | 1.851 | 3.89× | 3.86× |
| 100,000 | 4.590 | 21.79M | 5.006 | 19.98M | 24.362 | 5.31× | 4.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.310 | 5.79× |
| 1 | 5 | 0.250 | 0.702 | 2.81× |
| 1 | 10 | 0.433 | 1.435 | 3.31× |
| 10 | 1 | 0.047 | 0.128 | 2.71× |
| 10 | 5 | 0.182 | 0.678 | 3.73× |
| 10 | 10 | 0.421 | 1.463 | 3.48× |
| 100 | 1 | 0.055 | 0.223 | 4.09× |
| 100 | 5 | 0.201 | 1.217 | 6.05× |
| 100 | 10 | 0.412 | 2.542 | 6.17× |
| 1,000 | 1 | 0.099 | 0.396 | 4.02× |
| 1,000 | 5 | 0.206 | 1.625 | 7.88× |
| 1,000 | 10 | 0.489 | 3.224 | 6.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
