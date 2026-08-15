# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.55M | 0.010 | 102.83M | 0.037 | 2.76× | 3.76× |
| 10,000 | 0.112 | 89.53M | 0.106 | 94.05M | 0.126 | 1.13× | 1.19× |
| 100,000 | 1.105 | 90.50M | 1.145 | 87.33M | 0.803 | 0.73× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.092 | 1.05× |
| 1 | 5 | 0.275 | 0.450 | 1.64× |
| 1 | 10 | 0.385 | 0.987 | 2.56× |
| 10 | 1 | 0.045 | 0.091 | 2.01× |
| 10 | 5 | 0.193 | 0.462 | 2.40× |
| 10 | 10 | 0.402 | 0.919 | 2.29× |
| 100 | 1 | 0.049 | 0.089 | 1.81× |
| 100 | 5 | 0.222 | 0.474 | 2.14× |
| 100 | 10 | 0.381 | 0.987 | 2.59× |
| 1,000 | 1 | 0.054 | 0.101 | 1.87× |
| 1,000 | 5 | 0.206 | 0.482 | 2.34× |
| 1,000 | 10 | 0.449 | 1.132 | 2.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
