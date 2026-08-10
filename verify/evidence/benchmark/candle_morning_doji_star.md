# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.61M | 0.017 | 58.54M | 0.042 | 2.06× | 2.43× |
| 10,000 | 0.166 | 60.15M | 0.177 | 56.42M | 0.128 | 0.77× | 0.72× |
| 100,000 | 1.733 | 57.69M | 1.677 | 59.62M | 1.037 | 0.60× | 0.62× |
| 1,000,000 | 17.408 | 57.44M | 16.713 | 59.83M | 9.354 | 0.54× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.129 | 1.05× |
| 1 | 5 | 0.339 | 0.461 | 1.36× |
| 1 | 10 | 0.601 | 1.107 | 1.84× |
| 10 | 1 | 0.064 | 0.099 | 1.55× |
| 10 | 5 | 0.250 | 0.472 | 1.88× |
| 10 | 10 | 0.542 | 1.122 | 2.07× |
| 100 | 1 | 0.074 | 0.110 | 1.48× |
| 100 | 5 | 0.331 | 0.517 | 1.56× |
| 100 | 10 | 0.545 | 1.038 | 1.90× |
| 1,000 | 1 | 0.079 | 0.120 | 1.51× |
| 1,000 | 5 | 0.282 | 0.516 | 1.83× |
| 1,000 | 10 | 0.594 | 1.061 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
