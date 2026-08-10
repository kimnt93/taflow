# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.77M | 0.006 | 162.68M | 0.062 | 8.31× | 10.11× |
| 10,000 | 0.040 | 248.98M | 0.041 | 243.42M | 0.154 | 3.82× | 3.74× |
| 100,000 | 0.426 | 234.78M | 0.288 | 347.36M | 1.232 | 2.89× | 4.28× |
| 1,000,000 | 15.233 | 65.65M | 3.284 | 304.50M | 12.645 | 0.83× | 3.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.190 | 2.23× |
| 1 | 5 | 0.345 | 0.673 | 1.95× |
| 1 | 10 | 0.582 | 1.471 | 2.53× |
| 10 | 1 | 0.061 | 0.136 | 2.24× |
| 10 | 5 | 0.317 | 0.753 | 2.37× |
| 10 | 10 | 0.576 | 1.368 | 2.38× |
| 100 | 1 | 0.087 | 0.130 | 1.49× |
| 100 | 5 | 0.306 | 0.718 | 2.34× |
| 100 | 10 | 0.619 | 1.379 | 2.23× |
| 1,000 | 1 | 0.089 | 0.137 | 1.55× |
| 1,000 | 5 | 0.334 | 0.685 | 2.05× |
| 1,000 | 10 | 1.243 | 1.768 | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
