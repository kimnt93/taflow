# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.93M | 0.010 | 104.53M | 0.036 | 2.78× | 3.73× |
| 10,000 | 0.130 | 77.04M | 0.125 | 80.22M | 0.140 | 1.08× | 1.12× |
| 100,000 | 1.378 | 72.57M | 1.378 | 72.57M | 1.097 | 0.80× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.131 | 1.13× |
| 1 | 5 | 0.209 | 0.447 | 2.14× |
| 1 | 10 | 0.405 | 0.951 | 2.35× |
| 10 | 1 | 0.045 | 0.089 | 1.99× |
| 10 | 5 | 0.188 | 0.432 | 2.30× |
| 10 | 10 | 0.397 | 0.924 | 2.33× |
| 100 | 1 | 0.054 | 0.103 | 1.90× |
| 100 | 5 | 0.212 | 0.449 | 2.12× |
| 100 | 10 | 0.410 | 0.921 | 2.25× |
| 1,000 | 1 | 0.061 | 0.098 | 1.60× |
| 1,000 | 5 | 0.194 | 0.516 | 2.66× |
| 1,000 | 10 | 0.434 | 1.014 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
