# RollingSkew benchmark (`Skewness` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.61M | 0.045 | 22.25M | 0.230 | 4.74× | 5.12× |
| 10,000 | 0.437 | 22.90M | 0.430 | 23.25M | 0.694 | 1.59× | 1.61× |
| 100,000 | 4.299 | 23.26M | 4.204 | 23.79M | 6.681 | 1.55× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.239 | 2.30× |
| 1 | 5 | 0.263 | 1.132 | 4.31× |
| 1 | 10 | 0.433 | 2.186 | 5.05× |
| 10 | 1 | 0.054 | 0.212 | 3.92× |
| 10 | 5 | 0.192 | 1.222 | 6.38× |
| 10 | 10 | 0.385 | 2.220 | 5.77× |
| 100 | 1 | 0.054 | 0.221 | 4.08× |
| 100 | 5 | 0.194 | 1.300 | 6.71× |
| 100 | 10 | 0.432 | 2.290 | 5.30× |
| 1,000 | 1 | 0.091 | 0.263 | 2.91× |
| 1,000 | 5 | 0.206 | 1.503 | 7.29× |
| 1,000 | 10 | 0.454 | 2.795 | 6.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
