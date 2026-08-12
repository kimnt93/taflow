# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.14M | 0.016 | 62.99M | 0.032 | 1.64× | 2.03× |
| 10,000 | 0.140 | 71.67M | 0.137 | 73.05M | 0.086 | 0.62× | 0.63× |
| 100,000 | 1.370 | 72.99M | 1.365 | 73.28M | 0.596 | 0.43× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.113 | 1.42× |
| 1 | 5 | 0.325 | 0.490 | 1.51× |
| 1 | 10 | 0.546 | 0.933 | 1.71× |
| 10 | 1 | 0.058 | 0.086 | 1.48× |
| 10 | 5 | 0.266 | 0.460 | 1.73× |
| 10 | 10 | 0.550 | 0.947 | 1.72× |
| 100 | 1 | 0.053 | 0.089 | 1.66× |
| 100 | 5 | 0.277 | 0.456 | 1.64× |
| 100 | 10 | 0.534 | 0.918 | 1.72× |
| 1,000 | 1 | 0.069 | 0.105 | 1.53× |
| 1,000 | 5 | 0.254 | 0.469 | 1.85× |
| 1,000 | 10 | 0.570 | 0.985 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
