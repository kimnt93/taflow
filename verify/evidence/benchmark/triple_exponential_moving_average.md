# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.83M | 0.014 | 71.72M | 0.044 | 3.19× | 3.14× |
| 10,000 | 0.137 | 72.74M | 0.116 | 86.05M | 0.131 | 0.96× | 1.13× |
| 100,000 | 1.121 | 89.23M | 1.196 | 83.63M | 1.073 | 0.96× | 0.90× |
| 1,000,000 | 11.401 | 87.72M | 11.948 | 83.69M | 10.825 | 0.95× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.134 | 2.14× |
| 1 | 5 | 0.325 | 0.590 | 1.82× |
| 1 | 10 | 0.499 | 1.036 | 2.08× |
| 10 | 1 | 0.058 | 0.101 | 1.73× |
| 10 | 5 | 0.307 | 0.560 | 1.82× |
| 10 | 10 | 0.545 | 1.046 | 1.92× |
| 100 | 1 | 0.051 | 0.109 | 2.15× |
| 100 | 5 | 0.269 | 0.542 | 2.01× |
| 100 | 10 | 0.621 | 1.015 | 1.63× |
| 1,000 | 1 | 0.059 | 0.098 | 1.64× |
| 1,000 | 5 | 0.267 | 0.599 | 2.24× |
| 1,000 | 10 | 0.643 | 1.252 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
