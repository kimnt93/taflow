# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.29M | 0.009 | 112.48M | 0.030 | 2.53× | 3.34× |
| 10,000 | 0.077 | 130.68M | 0.074 | 134.47M | 0.094 | 1.23× | 1.27× |
| 100,000 | 0.870 | 114.94M | 0.796 | 125.60M | 0.622 | 0.72× | 0.78× |
| 1,000,000 | 8.611 | 116.13M | 8.159 | 122.56M | 6.448 | 0.75× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.100 | 0.89× |
| 1 | 5 | 0.359 | 0.465 | 1.30× |
| 1 | 10 | 0.565 | 0.920 | 1.63× |
| 10 | 1 | 0.054 | 0.096 | 1.79× |
| 10 | 5 | 0.324 | 0.525 | 1.62× |
| 10 | 10 | 0.585 | 1.009 | 1.73× |
| 100 | 1 | 0.062 | 0.092 | 1.48× |
| 100 | 5 | 0.294 | 0.440 | 1.50× |
| 100 | 10 | 0.537 | 0.962 | 1.79× |
| 1,000 | 1 | 0.070 | 0.098 | 1.39× |
| 1,000 | 5 | 0.285 | 0.608 | 2.14× |
| 1,000 | 10 | 0.740 | 1.016 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
