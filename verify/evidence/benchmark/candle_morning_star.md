# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.54M | 0.018 | 55.31M | 0.037 | 1.62× | 2.06× |
| 10,000 | 0.142 | 70.55M | 0.140 | 71.41M | 0.110 | 0.78× | 0.79× |
| 100,000 | 1.432 | 69.83M | 1.367 | 73.15M | 0.883 | 0.62× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.195 | 1.45× |
| 1 | 5 | 0.330 | 0.499 | 1.51× |
| 1 | 10 | 0.516 | 0.960 | 1.86× |
| 10 | 1 | 0.052 | 0.099 | 1.88× |
| 10 | 5 | 0.245 | 0.494 | 2.01× |
| 10 | 10 | 0.533 | 1.035 | 1.94× |
| 100 | 1 | 0.055 | 0.093 | 1.67× |
| 100 | 5 | 0.261 | 0.460 | 1.76× |
| 100 | 10 | 0.604 | 1.147 | 1.90× |
| 1,000 | 1 | 0.081 | 0.114 | 1.41× |
| 1,000 | 5 | 0.277 | 0.540 | 1.95× |
| 1,000 | 10 | 0.571 | 1.277 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
