# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.46M | 0.041 | 24.32M | 0.035 | 0.76× | 0.86× |
| 10,000 | 0.354 | 28.21M | 0.337 | 29.64M | 0.084 | 0.24× | 0.25× |
| 100,000 | 3.453 | 28.96M | 3.414 | 29.29M | 0.522 | 0.15× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.135 | 1.31× |
| 1 | 5 | 0.351 | 0.546 | 1.55× |
| 1 | 10 | 0.613 | 0.939 | 1.53× |
| 10 | 1 | 0.064 | 0.091 | 1.42× |
| 10 | 5 | 0.309 | 0.449 | 1.45× |
| 10 | 10 | 0.604 | 0.949 | 1.57× |
| 100 | 1 | 0.067 | 0.094 | 1.40× |
| 100 | 5 | 0.291 | 0.434 | 1.49× |
| 100 | 10 | 0.617 | 0.959 | 1.56× |
| 1,000 | 1 | 0.098 | 0.096 | 0.98× |
| 1,000 | 5 | 0.326 | 0.469 | 1.44× |
| 1,000 | 10 | 0.620 | 0.987 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
