# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.37M | 0.006 | 177.46M | 0.061 | 8.51× | 10.83× |
| 10,000 | 0.037 | 273.00M | 0.028 | 352.74M | 0.148 | 4.05× | 5.23× |
| 100,000 | 0.336 | 297.92M | 0.256 | 390.75M | 1.161 | 3.46× | 4.54× |
| 1,000,000 | 14.622 | 68.39M | 2.773 | 360.65M | 12.110 | 0.83× | 4.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.171 | 1.89× |
| 1 | 5 | 0.261 | 0.584 | 2.24× |
| 1 | 10 | 0.534 | 1.163 | 2.18× |
| 10 | 1 | 0.052 | 0.097 | 1.85× |
| 10 | 5 | 0.231 | 0.554 | 2.40× |
| 10 | 10 | 0.470 | 1.088 | 2.32× |
| 100 | 1 | 0.056 | 0.120 | 2.15× |
| 100 | 5 | 0.263 | 0.566 | 2.15× |
| 100 | 10 | 0.572 | 1.234 | 2.16× |
| 1,000 | 1 | 0.065 | 0.143 | 2.20× |
| 1,000 | 5 | 0.350 | 0.663 | 1.90× |
| 1,000 | 10 | 0.585 | 1.187 | 2.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
