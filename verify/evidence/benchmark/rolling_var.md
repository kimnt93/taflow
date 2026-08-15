# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 235.88M | 0.003 | 316.05M | 0.036 | 8.44× | 11.31× |
| 10,000 | 0.028 | 359.46M | 0.023 | 429.97M | 0.054 | 1.93× | 2.31× |
| 100,000 | 0.268 | 373.59M | 0.241 | 414.28M | 0.243 | 0.91× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.105 | 1.52× |
| 1 | 5 | 0.231 | 0.467 | 2.02× |
| 1 | 10 | 0.373 | 0.935 | 2.51× |
| 10 | 1 | 0.045 | 0.104 | 2.31× |
| 10 | 5 | 0.194 | 0.457 | 2.36× |
| 10 | 10 | 0.386 | 0.944 | 2.45× |
| 100 | 1 | 0.041 | 0.091 | 2.25× |
| 100 | 5 | 0.183 | 0.471 | 2.57× |
| 100 | 10 | 0.441 | 0.967 | 2.19× |
| 1,000 | 1 | 0.047 | 0.089 | 1.91× |
| 1,000 | 5 | 0.193 | 0.460 | 2.39× |
| 1,000 | 10 | 0.421 | 1.028 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
