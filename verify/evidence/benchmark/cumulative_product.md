# CumulativeProduct benchmark (`numpy.cumprod` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 221.12M | 0.004 | 248.65M | 0.018 | 3.95× | 4.44× |
| 10,000 | 0.015 | 681.42M | 0.012 | 809.37M | 0.038 | 2.56× | 3.04× |
| 100,000 | 0.132 | 755.48M | 0.096 | 1.05G | 0.220 | 1.66× | 2.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.073 | 0.73× |
| 1 | 5 | 0.284 | 0.402 | 1.42× |
| 1 | 10 | 0.499 | 0.669 | 1.34× |
| 10 | 1 | 0.054 | 0.072 | 1.34× |
| 10 | 5 | 0.243 | 0.320 | 1.31× |
| 10 | 10 | 0.500 | 0.697 | 1.39× |
| 100 | 1 | 0.049 | 0.070 | 1.43× |
| 100 | 5 | 0.249 | 0.341 | 1.37× |
| 100 | 10 | 0.503 | 0.672 | 1.34× |
| 1,000 | 1 | 0.051 | 0.063 | 1.25× |
| 1,000 | 5 | 0.234 | 0.363 | 1.55× |
| 1,000 | 10 | 0.476 | 0.760 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
