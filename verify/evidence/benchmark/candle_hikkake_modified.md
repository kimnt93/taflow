# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.44M | 0.004 | 244.78M | 0.031 | 4.24× | 7.56× |
| 10,000 | 0.065 | 155.02M | 0.063 | 158.54M | 0.095 | 1.48× | 1.51× |
| 100,000 | 0.600 | 166.73M | 0.571 | 175.16M | 0.530 | 0.88× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.104 | 1.32× |
| 1 | 5 | 0.317 | 0.494 | 1.56× |
| 1 | 10 | 0.389 | 0.853 | 2.19× |
| 10 | 1 | 0.040 | 0.086 | 2.14× |
| 10 | 5 | 0.182 | 0.404 | 2.22× |
| 10 | 10 | 0.386 | 0.886 | 2.30× |
| 100 | 1 | 0.043 | 0.082 | 1.91× |
| 100 | 5 | 0.185 | 0.425 | 2.29× |
| 100 | 10 | 0.391 | 0.866 | 2.21× |
| 1,000 | 1 | 0.046 | 0.093 | 2.03× |
| 1,000 | 5 | 0.186 | 0.448 | 2.40× |
| 1,000 | 10 | 0.390 | 0.919 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
