# RollingCalmar benchmark (`rolling calmar on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.88M | 0.044 | 22.59M | 0.282 | 5.89× | 6.37× |
| 10,000 | 0.417 | 23.96M | 0.406 | 24.60M | 1.464 | 3.51× | 3.60× |
| 100,000 | 4.535 | 22.05M | 4.034 | 24.79M | 17.861 | 3.94× | 4.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.110 | 1.31× |
| 1 | 5 | 0.224 | 0.440 | 1.96× |
| 1 | 10 | 0.379 | 0.997 | 2.63× |
| 10 | 1 | 0.045 | 0.090 | 1.99× |
| 10 | 5 | 0.223 | 0.461 | 2.07× |
| 10 | 10 | 0.395 | 0.866 | 2.20× |
| 100 | 1 | 0.048 | 0.204 | 4.22× |
| 100 | 5 | 0.190 | 1.058 | 5.56× |
| 100 | 10 | 0.423 | 2.166 | 5.12× |
| 1,000 | 1 | 0.089 | 0.324 | 3.65× |
| 1,000 | 5 | 0.230 | 1.261 | 5.47× |
| 1,000 | 10 | 0.462 | 2.619 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
