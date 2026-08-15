# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.65M | 0.011 | 88.69M | 0.040 | 2.79× | 3.55× |
| 10,000 | 0.162 | 61.91M | 0.158 | 63.44M | 0.176 | 1.09× | 1.12× |
| 100,000 | 1.702 | 58.77M | 1.713 | 58.36M | 1.477 | 0.87× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.108 | 0.96× |
| 1 | 5 | 0.341 | 0.514 | 1.51× |
| 1 | 10 | 0.431 | 0.917 | 2.13× |
| 10 | 1 | 0.045 | 0.085 | 1.90× |
| 10 | 5 | 0.174 | 0.433 | 2.49× |
| 10 | 10 | 0.428 | 0.998 | 2.33× |
| 100 | 1 | 0.046 | 0.093 | 2.01× |
| 100 | 5 | 0.211 | 0.440 | 2.08× |
| 100 | 10 | 0.379 | 1.036 | 2.74× |
| 1,000 | 1 | 0.072 | 0.110 | 1.53× |
| 1,000 | 5 | 0.204 | 0.521 | 2.55× |
| 1,000 | 10 | 0.442 | 1.093 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
