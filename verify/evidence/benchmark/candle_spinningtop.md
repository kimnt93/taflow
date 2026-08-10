# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.32M | 0.022 | 45.79M | 0.058 | 2.88× | 2.67× |
| 10,000 | 0.166 | 60.20M | 0.128 | 77.90M | 0.148 | 0.89× | 1.15× |
| 100,000 | 1.357 | 73.70M | 1.320 | 75.74M | 1.234 | 0.91× | 0.93× |
| 1,000,000 | 13.643 | 73.30M | 13.261 | 75.41M | 11.771 | 0.86× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.115 | 1.21× |
| 1 | 5 | 0.320 | 0.566 | 1.77× |
| 1 | 10 | 0.723 | 1.210 | 1.67× |
| 10 | 1 | 0.070 | 0.099 | 1.40× |
| 10 | 5 | 0.324 | 0.531 | 1.64× |
| 10 | 10 | 0.679 | 1.196 | 1.76× |
| 100 | 1 | 0.085 | 0.107 | 1.26× |
| 100 | 5 | 0.374 | 0.576 | 1.54× |
| 100 | 10 | 0.710 | 1.284 | 1.81× |
| 1,000 | 1 | 0.102 | 0.122 | 1.20× |
| 1,000 | 5 | 0.341 | 0.614 | 1.80× |
| 1,000 | 10 | 0.750 | 1.390 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
