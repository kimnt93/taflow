# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.65M | 0.047 | 21.40M | 0.036 | 0.75× | 0.77× |
| 10,000 | 0.390 | 25.61M | 0.347 | 28.78M | 0.077 | 0.20× | 0.22× |
| 100,000 | 3.398 | 29.43M | 3.413 | 29.30M | 0.491 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.119 | 1.22× |
| 1 | 5 | 0.353 | 0.487 | 1.38× |
| 1 | 10 | 0.600 | 0.961 | 1.60× |
| 10 | 1 | 0.072 | 0.091 | 1.26× |
| 10 | 5 | 0.289 | 0.438 | 1.51× |
| 10 | 10 | 0.605 | 0.945 | 1.56× |
| 100 | 1 | 0.075 | 0.097 | 1.30× |
| 100 | 5 | 0.290 | 0.431 | 1.49× |
| 100 | 10 | 0.619 | 0.934 | 1.51× |
| 1,000 | 1 | 0.112 | 0.101 | 0.91× |
| 1,000 | 5 | 0.293 | 0.498 | 1.70× |
| 1,000 | 10 | 0.630 | 1.018 | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
