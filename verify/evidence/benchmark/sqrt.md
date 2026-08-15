# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 401.68M | 0.001 | 713.21M | 0.030 | 12.18× | 21.62× |
| 10,000 | 0.011 | 932.12M | 0.008 | 1.25G | 0.041 | 3.79× | 5.09× |
| 100,000 | 0.098 | 1.02G | 0.076 | 1.31G | 0.175 | 1.79× | 2.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.137 | 1.81× |
| 1 | 5 | 0.220 | 0.455 | 2.07× |
| 1 | 10 | 0.392 | 1.003 | 2.56× |
| 10 | 1 | 0.045 | 0.093 | 2.07× |
| 10 | 5 | 0.233 | 0.492 | 2.11× |
| 10 | 10 | 0.373 | 0.904 | 2.43× |
| 100 | 1 | 0.055 | 0.088 | 1.58× |
| 100 | 5 | 0.217 | 0.474 | 2.19× |
| 100 | 10 | 0.439 | 0.957 | 2.18× |
| 1,000 | 1 | 0.044 | 0.084 | 1.92× |
| 1,000 | 5 | 0.178 | 0.485 | 2.73× |
| 1,000 | 10 | 0.470 | 1.063 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
