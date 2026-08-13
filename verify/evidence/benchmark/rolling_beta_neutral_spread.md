# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.674 | 1.48M | 0.678 | 1.48M | 0.203 | 0.30× | 0.30× |
| 10,000 | 6.699 | 1.49M | 6.679 | 1.50M | 0.895 | 0.13× | 0.13× |
| 100,000 | 68.185 | 1.47M | 67.597 | 1.48M | 7.856 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.250 | 1.83× |
| 1 | 5 | 0.480 | 1.096 | 2.28× |
| 1 | 10 | 0.659 | 2.200 | 3.34× |
| 10 | 1 | 0.068 | 0.212 | 3.10× |
| 10 | 5 | 0.309 | 1.215 | 3.93× |
| 10 | 10 | 0.644 | 2.250 | 3.49× |
| 100 | 1 | 0.130 | 0.225 | 1.74× |
| 100 | 5 | 0.324 | 1.254 | 3.87× |
| 100 | 10 | 0.738 | 2.335 | 3.16× |
| 1,000 | 1 | 0.765 | 0.297 | 0.39× |
| 1,000 | 5 | 0.916 | 1.678 | 1.83× |
| 1,000 | 10 | 1.713 | 3.534 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
