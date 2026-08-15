# RollingCoefficientOfDetermination benchmark (`rolling squared correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.33M | 0.050 | 20.02M | 0.301 | 6.12× | 6.03× |
| 10,000 | 0.487 | 20.53M | 0.488 | 20.48M | 1.958 | 4.02× | 4.01× |
| 100,000 | 4.920 | 20.33M | 4.760 | 21.01M | 25.228 | 5.13× | 5.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.174 | 2.52× |
| 1 | 5 | 0.311 | 0.744 | 2.39× |
| 1 | 10 | 0.425 | 1.343 | 3.16× |
| 10 | 1 | 0.046 | 0.139 | 3.03× |
| 10 | 5 | 0.188 | 0.649 | 3.46× |
| 10 | 10 | 0.462 | 1.394 | 3.01× |
| 100 | 1 | 0.051 | 0.227 | 4.43× |
| 100 | 5 | 0.201 | 1.224 | 6.08× |
| 100 | 10 | 0.485 | 2.489 | 5.13× |
| 1,000 | 1 | 0.096 | 0.391 | 4.05× |
| 1,000 | 5 | 0.223 | 1.580 | 7.09× |
| 1,000 | 10 | 0.477 | 3.317 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
