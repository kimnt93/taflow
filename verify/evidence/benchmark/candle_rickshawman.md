# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.05M | 0.014 | 72.67M | 0.038 | 2.18× | 2.73× |
| 10,000 | 0.111 | 90.36M | 0.108 | 92.91M | 0.121 | 1.10× | 1.13× |
| 100,000 | 1.118 | 89.47M | 1.139 | 87.76M | 0.983 | 0.88× | 0.86× |
| 1,000,000 | 11.442 | 87.39M | 11.769 | 84.97M | 9.494 | 0.83× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.157 | 1.77× |
| 1 | 5 | 0.434 | 0.465 | 1.07× |
| 1 | 10 | 0.523 | 0.967 | 1.85× |
| 10 | 1 | 0.062 | 0.098 | 1.58× |
| 10 | 5 | 0.255 | 0.445 | 1.74× |
| 10 | 10 | 0.556 | 0.974 | 1.75× |
| 100 | 1 | 0.057 | 0.096 | 1.69× |
| 100 | 5 | 0.247 | 0.434 | 1.76× |
| 100 | 10 | 0.564 | 0.914 | 1.62× |
| 1,000 | 1 | 0.064 | 0.099 | 1.55× |
| 1,000 | 5 | 0.296 | 0.486 | 1.64× |
| 1,000 | 10 | 0.620 | 1.063 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
