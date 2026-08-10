# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.32M | 0.010 | 101.37M | 0.037 | 2.29× | 3.79× |
| 10,000 | 0.081 | 123.88M | 0.074 | 134.69M | 0.099 | 1.23× | 1.33× |
| 100,000 | 0.842 | 118.78M | 0.812 | 123.11M | 0.622 | 0.74× | 0.77× |
| 1,000,000 | 8.909 | 112.25M | 8.778 | 113.92M | 6.225 | 0.70× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.140 | 1.49× |
| 1 | 5 | 0.308 | 0.544 | 1.77× |
| 1 | 10 | 0.581 | 0.936 | 1.61× |
| 10 | 1 | 0.058 | 0.110 | 1.88× |
| 10 | 5 | 0.294 | 0.571 | 1.94× |
| 10 | 10 | 0.655 | 1.046 | 1.60× |
| 100 | 1 | 0.069 | 0.090 | 1.31× |
| 100 | 5 | 0.284 | 0.467 | 1.64× |
| 100 | 10 | 0.621 | 1.036 | 1.67× |
| 1,000 | 1 | 0.074 | 0.099 | 1.34× |
| 1,000 | 5 | 0.267 | 0.495 | 1.86× |
| 1,000 | 10 | 0.674 | 1.127 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
