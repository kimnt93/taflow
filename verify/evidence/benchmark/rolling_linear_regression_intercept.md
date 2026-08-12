# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 60.97M | 0.016 | 62.48M | 0.044 | 2.67× | 2.74× |
| 10,000 | 0.134 | 74.64M | 0.127 | 78.90M | 0.160 | 1.19× | 1.26× |
| 100,000 | 1.428 | 70.01M | 1.272 | 78.61M | 1.333 | 0.93× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.108 | 1.64× |
| 1 | 5 | 0.300 | 0.468 | 1.56× |
| 1 | 10 | 0.493 | 1.036 | 2.10× |
| 10 | 1 | 0.054 | 0.094 | 1.74× |
| 10 | 5 | 0.225 | 0.441 | 1.96× |
| 10 | 10 | 0.467 | 1.000 | 2.14× |
| 100 | 1 | 0.056 | 0.098 | 1.75× |
| 100 | 5 | 0.252 | 0.471 | 1.87× |
| 100 | 10 | 0.502 | 0.957 | 1.91× |
| 1,000 | 1 | 0.073 | 0.101 | 1.38× |
| 1,000 | 5 | 0.239 | 0.556 | 2.33× |
| 1,000 | 10 | 0.521 | 1.106 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
