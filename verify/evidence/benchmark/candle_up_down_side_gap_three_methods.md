# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.24M | 0.015 | 66.95M | 0.032 | 1.91× | 2.16× |
| 10,000 | 0.121 | 82.98M | 0.116 | 86.47M | 0.091 | 0.75× | 0.79× |
| 100,000 | 1.134 | 88.17M | 1.124 | 88.93M | 0.641 | 0.57× | 0.57× |
| 1,000,000 | 11.617 | 86.08M | 11.195 | 89.33M | 6.188 | 0.53× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.127 | 1.01× |
| 1 | 5 | 0.305 | 0.481 | 1.58× |
| 1 | 10 | 0.521 | 0.951 | 1.83× |
| 10 | 1 | 0.054 | 0.091 | 1.67× |
| 10 | 5 | 0.245 | 0.484 | 1.97× |
| 10 | 10 | 0.493 | 0.895 | 1.82× |
| 100 | 1 | 0.055 | 0.088 | 1.62× |
| 100 | 5 | 0.249 | 0.462 | 1.85× |
| 100 | 10 | 0.542 | 0.974 | 1.80× |
| 1,000 | 1 | 0.071 | 0.100 | 1.42× |
| 1,000 | 5 | 0.246 | 0.449 | 1.83× |
| 1,000 | 10 | 0.541 | 1.077 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
