# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.29M | 0.011 | 93.87M | 0.037 | 2.86× | 3.52× |
| 10,000 | 0.074 | 134.71M | 0.070 | 142.97M | 0.086 | 1.16× | 1.23× |
| 100,000 | 0.683 | 146.40M | 0.641 | 156.07M | 0.587 | 0.86× | 0.92× |
| 1,000,000 | 7.647 | 130.77M | 6.799 | 147.07M | 6.062 | 0.79× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.125 | 0.93× |
| 1 | 5 | 0.340 | 0.504 | 1.48× |
| 1 | 10 | 0.484 | 0.929 | 1.92× |
| 10 | 1 | 0.048 | 0.090 | 1.86× |
| 10 | 5 | 0.236 | 0.427 | 1.81× |
| 10 | 10 | 0.487 | 0.915 | 1.88× |
| 100 | 1 | 0.048 | 0.094 | 1.95× |
| 100 | 5 | 0.227 | 0.445 | 1.96× |
| 100 | 10 | 0.506 | 0.925 | 1.83× |
| 1,000 | 1 | 0.067 | 0.100 | 1.51× |
| 1,000 | 5 | 0.246 | 0.471 | 1.91× |
| 1,000 | 10 | 0.510 | 1.018 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
