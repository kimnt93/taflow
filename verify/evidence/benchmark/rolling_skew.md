# RollingSkew benchmark (`Skewness` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.97M | 0.044 | 22.90M | 0.201 | 4.43× | 4.61× |
| 10,000 | 0.430 | 23.23M | 0.421 | 23.78M | 0.751 | 1.74× | 1.79× |
| 100,000 | 4.343 | 23.03M | 4.494 | 22.25M | 6.401 | 1.47× | 1.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.221 | 2.62× |
| 1 | 5 | 0.217 | 1.115 | 5.13× |
| 1 | 10 | 0.408 | 2.231 | 5.47× |
| 10 | 1 | 0.042 | 0.220 | 5.29× |
| 10 | 5 | 0.216 | 1.320 | 6.12× |
| 10 | 10 | 0.411 | 2.267 | 5.51× |
| 100 | 1 | 0.056 | 0.213 | 3.82× |
| 100 | 5 | 0.208 | 1.268 | 6.10× |
| 100 | 10 | 0.456 | 2.343 | 5.14× |
| 1,000 | 1 | 0.092 | 0.291 | 3.17× |
| 1,000 | 5 | 0.202 | 1.527 | 7.57× |
| 1,000 | 10 | 0.467 | 2.901 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
