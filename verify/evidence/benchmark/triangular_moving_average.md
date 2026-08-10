# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.46M | 0.007 | 135.55M | 0.043 | 5.02× | 5.79× |
| 10,000 | 0.049 | 203.60M | 0.046 | 215.13M | 0.069 | 1.40× | 1.48× |
| 100,000 | 0.467 | 214.10M | 0.432 | 231.73M | 0.378 | 0.81× | 0.88× |
| 1,000,000 | 6.090 | 164.21M | 4.774 | 209.46M | 3.388 | 0.56× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.128 | 1.84× |
| 1 | 5 | 0.279 | 0.506 | 1.81× |
| 1 | 10 | 0.538 | 1.197 | 2.22× |
| 10 | 1 | 0.068 | 0.134 | 1.98× |
| 10 | 5 | 0.281 | 0.511 | 1.82× |
| 10 | 10 | 0.505 | 1.137 | 2.25× |
| 100 | 1 | 0.061 | 0.099 | 1.64× |
| 100 | 5 | 0.309 | 0.575 | 1.86× |
| 100 | 10 | 0.564 | 1.093 | 1.94× |
| 1,000 | 1 | 0.069 | 0.120 | 1.74× |
| 1,000 | 5 | 0.328 | 0.614 | 1.88× |
| 1,000 | 10 | 0.567 | 1.195 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
