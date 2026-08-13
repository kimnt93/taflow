# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.103 | 9.73M | 0.093 | 10.77M | 0.032 | 0.31× | 0.35× |
| 10,000 | 0.815 | 12.26M | 0.861 | 11.62M | 0.113 | 0.14× | 0.13× |
| 100,000 | 8.025 | 12.46M | 7.962 | 12.56M | 1.064 | 0.13× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.204 | 0.198 | 0.97× |
| 1 | 5 | 0.415 | 0.492 | 1.19× |
| 1 | 10 | 0.675 | 0.898 | 1.33× |
| 10 | 1 | 0.070 | 0.090 | 1.28× |
| 10 | 5 | 0.311 | 0.434 | 1.40× |
| 10 | 10 | 0.654 | 0.881 | 1.35× |
| 100 | 1 | 0.077 | 0.088 | 1.14× |
| 100 | 5 | 0.314 | 0.436 | 1.39× |
| 100 | 10 | 0.630 | 0.876 | 1.39× |
| 1,000 | 1 | 0.157 | 0.094 | 0.60× |
| 1,000 | 5 | 0.350 | 0.471 | 1.34× |
| 1,000 | 10 | 0.667 | 1.001 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
