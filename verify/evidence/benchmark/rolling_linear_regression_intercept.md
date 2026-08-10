# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.61M | 0.018 | 57.05M | 0.047 | 2.77× | 2.69× |
| 10,000 | 0.145 | 68.82M | 0.146 | 68.37M | 0.180 | 1.24× | 1.23× |
| 100,000 | 1.538 | 65.03M | 1.619 | 61.77M | 1.535 | 1.00× | 0.95× |
| 1,000,000 | 15.205 | 65.77M | 14.055 | 71.15M | 15.777 | 1.04× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.136 | 0.86× |
| 1 | 5 | 0.263 | 0.506 | 1.92× |
| 1 | 10 | 0.483 | 1.061 | 2.20× |
| 10 | 1 | 0.057 | 0.100 | 1.75× |
| 10 | 5 | 0.248 | 0.500 | 2.01× |
| 10 | 10 | 0.485 | 1.048 | 2.16× |
| 100 | 1 | 0.066 | 0.113 | 1.72× |
| 100 | 5 | 0.265 | 0.503 | 1.90× |
| 100 | 10 | 0.530 | 1.032 | 1.95× |
| 1,000 | 1 | 0.063 | 0.115 | 1.83× |
| 1,000 | 5 | 0.265 | 0.569 | 2.15× |
| 1,000 | 10 | 0.576 | 1.150 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
