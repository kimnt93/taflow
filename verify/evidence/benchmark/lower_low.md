# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.30M | 0.041 | 24.63M | 0.022 | 0.49× | 0.54× |
| 10,000 | 0.335 | 29.86M | 0.345 | 28.99M | 0.043 | 0.13× | 0.12× |
| 100,000 | 3.463 | 28.88M | 3.160 | 31.65M | 0.226 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.146 | 1.50× |
| 1 | 5 | 0.453 | 0.353 | 0.78× |
| 1 | 10 | 0.600 | 0.724 | 1.21× |
| 10 | 1 | 0.063 | 0.076 | 1.22× |
| 10 | 5 | 0.307 | 0.352 | 1.15× |
| 10 | 10 | 0.586 | 0.739 | 1.26× |
| 100 | 1 | 0.068 | 0.076 | 1.11× |
| 100 | 5 | 0.294 | 0.362 | 1.23× |
| 100 | 10 | 0.594 | 0.754 | 1.27× |
| 1,000 | 1 | 0.103 | 0.075 | 0.72× |
| 1,000 | 5 | 0.284 | 0.526 | 1.85× |
| 1,000 | 10 | 0.621 | 1.113 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
