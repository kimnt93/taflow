# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.88M | 0.008 | 128.15M | 0.033 | 3.44× | 4.24× |
| 10,000 | 0.103 | 97.11M | 0.099 | 100.76M | 0.088 | 0.85× | 0.89× |
| 100,000 | 1.091 | 91.70M | 1.026 | 97.49M | 0.630 | 0.58× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.110 | 1.16× |
| 1 | 5 | 0.253 | 0.457 | 1.81× |
| 1 | 10 | 0.384 | 0.899 | 2.34× |
| 10 | 1 | 0.040 | 0.085 | 2.14× |
| 10 | 5 | 0.219 | 0.471 | 2.15× |
| 10 | 10 | 0.397 | 0.916 | 2.31× |
| 100 | 1 | 0.043 | 0.097 | 2.25× |
| 100 | 5 | 0.200 | 0.434 | 2.16× |
| 100 | 10 | 0.435 | 0.958 | 2.20× |
| 1,000 | 1 | 0.061 | 0.099 | 1.63× |
| 1,000 | 5 | 0.212 | 0.475 | 2.25× |
| 1,000 | 10 | 0.428 | 1.024 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
