# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.68M | 0.029 | 34.65M | 0.181 | 5.92× | 6.28× |
| 10,000 | 0.287 | 34.83M | 0.296 | 33.73M | 0.687 | 2.39× | 2.32× |
| 100,000 | 2.855 | 35.02M | 2.869 | 34.86M | 5.632 | 1.97× | 1.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.177 | 0.244 | 1.38× |
| 1 | 5 | 0.332 | 1.079 | 3.25× |
| 1 | 10 | 0.433 | 2.239 | 5.17× |
| 10 | 1 | 0.046 | 0.214 | 4.62× |
| 10 | 5 | 0.187 | 1.343 | 7.18× |
| 10 | 10 | 0.408 | 2.214 | 5.43× |
| 100 | 1 | 0.045 | 0.218 | 4.85× |
| 100 | 5 | 0.229 | 1.308 | 5.70× |
| 100 | 10 | 0.427 | 2.242 | 5.25× |
| 1,000 | 1 | 0.082 | 0.281 | 3.44× |
| 1,000 | 5 | 0.209 | 1.530 | 7.33× |
| 1,000 | 10 | 0.440 | 3.042 | 6.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
