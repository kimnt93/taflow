# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.39M | 0.010 | 101.25M | 0.034 | 2.58× | 3.47× |
| 10,000 | 0.089 | 112.39M | 0.083 | 120.08M | 0.094 | 1.06× | 1.13× |
| 100,000 | 0.989 | 101.07M | 0.954 | 104.84M | 0.713 | 0.72× | 0.75× |
| 1,000,000 | 9.814 | 101.90M | 9.384 | 106.56M | 8.351 | 0.85× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.202 | 1.42× |
| 1 | 5 | 0.285 | 0.477 | 1.67× |
| 1 | 10 | 0.537 | 0.971 | 1.81× |
| 10 | 1 | 0.066 | 0.092 | 1.39× |
| 10 | 5 | 0.255 | 0.423 | 1.66× |
| 10 | 10 | 0.520 | 0.967 | 1.86× |
| 100 | 1 | 0.075 | 0.089 | 1.18× |
| 100 | 5 | 0.313 | 0.500 | 1.60× |
| 100 | 10 | 0.564 | 0.920 | 1.63× |
| 1,000 | 1 | 0.074 | 0.111 | 1.50× |
| 1,000 | 5 | 0.311 | 0.562 | 1.81× |
| 1,000 | 10 | 0.594 | 0.994 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
