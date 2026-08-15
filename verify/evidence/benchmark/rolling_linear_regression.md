# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.04M | 0.014 | 70.12M | 0.044 | 2.83× | 3.09× |
| 10,000 | 0.136 | 73.64M | 0.130 | 77.09M | 0.161 | 1.18× | 1.24× |
| 100,000 | 1.319 | 75.83M | 1.265 | 79.07M | 1.338 | 1.01× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.122 | 1.82× |
| 1 | 5 | 0.263 | 0.464 | 1.76× |
| 1 | 10 | 0.383 | 0.966 | 2.52× |
| 10 | 1 | 0.042 | 0.102 | 2.43× |
| 10 | 5 | 0.181 | 0.453 | 2.51× |
| 10 | 10 | 0.389 | 0.916 | 2.35× |
| 100 | 1 | 0.041 | 0.091 | 2.23× |
| 100 | 5 | 0.192 | 0.488 | 2.54× |
| 100 | 10 | 0.426 | 0.932 | 2.19× |
| 1,000 | 1 | 0.062 | 0.121 | 1.94× |
| 1,000 | 5 | 0.192 | 0.500 | 2.61× |
| 1,000 | 10 | 0.439 | 1.083 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
