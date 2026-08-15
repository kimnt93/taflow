# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.98M | 0.005 | 216.48M | 0.042 | 5.61× | 9.20× |
| 10,000 | 0.085 | 118.25M | 0.078 | 127.54M | 0.172 | 2.04× | 2.20× |
| 100,000 | 1.144 | 87.43M | 1.090 | 91.78M | 1.454 | 1.27× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.116 | 1.38× |
| 1 | 5 | 0.224 | 0.510 | 2.28× |
| 1 | 10 | 0.404 | 0.913 | 2.26× |
| 10 | 1 | 0.044 | 0.091 | 2.04× |
| 10 | 5 | 0.184 | 0.436 | 2.37× |
| 10 | 10 | 0.417 | 0.943 | 2.26× |
| 100 | 1 | 0.048 | 0.092 | 1.91× |
| 100 | 5 | 0.176 | 0.435 | 2.47× |
| 100 | 10 | 0.405 | 0.969 | 2.39× |
| 1,000 | 1 | 0.068 | 0.112 | 1.64× |
| 1,000 | 5 | 0.199 | 0.534 | 2.69× |
| 1,000 | 10 | 0.424 | 1.084 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
