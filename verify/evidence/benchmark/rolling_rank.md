# RollingRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.73M | 0.020 | 49.27M | 0.143 | 5.38× | 7.02× |
| 10,000 | 0.173 | 57.93M | 0.173 | 57.85M | 0.762 | 4.41× | 4.41× |
| 100,000 | 1.745 | 57.30M | 1.693 | 59.05M | 7.400 | 4.24× | 4.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.160 | 1.39× |
| 1 | 5 | 0.418 | 0.667 | 1.60× |
| 1 | 10 | 0.519 | 1.109 | 2.14× |
| 10 | 1 | 0.051 | 0.109 | 2.11× |
| 10 | 5 | 0.222 | 0.511 | 2.30× |
| 10 | 10 | 0.490 | 1.163 | 2.37× |
| 100 | 1 | 0.056 | 0.159 | 2.87× |
| 100 | 5 | 0.239 | 0.800 | 3.34× |
| 100 | 10 | 0.490 | 1.716 | 3.50× |
| 1,000 | 1 | 0.070 | 0.219 | 3.13× |
| 1,000 | 5 | 0.244 | 0.942 | 3.86× |
| 1,000 | 10 | 0.508 | 2.166 | 4.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
