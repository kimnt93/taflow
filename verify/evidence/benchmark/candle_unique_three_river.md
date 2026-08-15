# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.52M | 0.003 | 382.68M | 0.032 | 5.36× | 12.32× |
| 10,000 | 0.048 | 208.62M | 0.041 | 243.75M | 0.081 | 1.68× | 1.97× |
| 100,000 | 0.668 | 149.69M | 0.671 | 149.13M | 0.572 | 0.86× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.104 | 1.13× |
| 1 | 5 | 0.314 | 0.474 | 1.51× |
| 1 | 10 | 0.449 | 0.913 | 2.03× |
| 10 | 1 | 0.040 | 0.083 | 2.06× |
| 10 | 5 | 0.170 | 0.413 | 2.44× |
| 10 | 10 | 0.375 | 0.960 | 2.56× |
| 100 | 1 | 0.043 | 0.092 | 2.17× |
| 100 | 5 | 0.187 | 0.450 | 2.40× |
| 100 | 10 | 0.405 | 0.938 | 2.31× |
| 1,000 | 1 | 0.047 | 0.097 | 2.04× |
| 1,000 | 5 | 0.234 | 0.519 | 2.22× |
| 1,000 | 10 | 0.438 | 1.017 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
