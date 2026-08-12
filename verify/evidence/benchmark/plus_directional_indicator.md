# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.98M | 0.010 | 97.60M | 0.040 | 3.32× | 3.95× |
| 10,000 | 0.064 | 155.88M | 0.061 | 163.52M | 0.096 | 1.50× | 1.58× |
| 100,000 | 0.625 | 159.88M | 0.575 | 173.96M | 0.681 | 1.09× | 1.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.111 | 1.58× |
| 1 | 5 | 0.369 | 0.474 | 1.28× |
| 1 | 10 | 0.517 | 1.055 | 2.04× |
| 10 | 1 | 0.060 | 0.092 | 1.54× |
| 10 | 5 | 0.239 | 0.454 | 1.90× |
| 10 | 10 | 0.489 | 0.936 | 1.91× |
| 100 | 1 | 0.052 | 0.107 | 2.05× |
| 100 | 5 | 0.290 | 0.543 | 1.87× |
| 100 | 10 | 0.510 | 0.965 | 1.89× |
| 1,000 | 1 | 0.062 | 0.099 | 1.61× |
| 1,000 | 5 | 0.278 | 0.561 | 2.02× |
| 1,000 | 10 | 0.593 | 1.083 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
