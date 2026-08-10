# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.91M | 0.077 | 12.97M | 0.224 | 4.01× | 2.90× |
| 10,000 | 0.499 | 20.03M | 0.506 | 19.78M | 1.039 | 2.08× | 2.05× |
| 100,000 | 5.303 | 18.86M | 5.161 | 19.38M | 8.513 | 1.61× | 1.65× |
| 1,000,000 | 48.465 | 20.63M | 49.903 | 20.04M | 86.178 | 1.78× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.259 | 3.12× |
| 1 | 5 | 0.269 | 1.341 | 4.99× |
| 1 | 10 | 0.579 | 2.417 | 4.18× |
| 10 | 1 | 0.053 | 0.209 | 3.96× |
| 10 | 5 | 0.255 | 1.423 | 5.59× |
| 10 | 10 | 0.535 | 2.553 | 4.77× |
| 100 | 1 | 0.067 | 0.294 | 4.39× |
| 100 | 5 | 0.316 | 1.504 | 4.75× |
| 100 | 10 | 0.562 | 2.617 | 4.65× |
| 1,000 | 1 | 0.106 | 0.298 | 2.83× |
| 1,000 | 5 | 0.270 | 1.712 | 6.35× |
| 1,000 | 10 | 0.601 | 3.400 | 5.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
