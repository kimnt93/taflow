# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.93M | 0.007 | 137.52M | 0.045 | 5.63× | 6.25× |
| 10,000 | 0.065 | 154.50M | 0.054 | 184.81M | 0.089 | 1.38× | 1.65× |
| 100,000 | 0.559 | 178.89M | 0.521 | 191.99M | 0.588 | 1.05× | 1.13× |
| 1,000,000 | 5.688 | 175.80M | 5.561 | 179.81M | 5.617 | 0.99× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.109 | 1.19× |
| 1 | 5 | 0.334 | 0.519 | 1.55× |
| 1 | 10 | 0.483 | 0.947 | 1.96× |
| 10 | 1 | 0.051 | 0.089 | 1.74× |
| 10 | 5 | 0.212 | 0.438 | 2.06× |
| 10 | 10 | 0.440 | 0.920 | 2.09× |
| 100 | 1 | 0.048 | 0.099 | 2.06× |
| 100 | 5 | 0.212 | 0.428 | 2.02× |
| 100 | 10 | 0.431 | 0.913 | 2.12× |
| 1,000 | 1 | 0.054 | 0.099 | 1.82× |
| 1,000 | 5 | 0.224 | 0.469 | 2.09× |
| 1,000 | 10 | 0.463 | 0.993 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
