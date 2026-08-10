# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.50M | 0.005 | 210.53M | 0.051 | 7.56× | 10.64× |
| 10,000 | 0.032 | 311.59M | 0.025 | 400.42M | 0.142 | 4.41× | 5.67× |
| 100,000 | 0.285 | 351.48M | 0.243 | 411.57M | 1.023 | 3.59× | 4.21× |
| 1,000,000 | 12.276 | 81.46M | 2.365 | 422.92M | 10.295 | 0.84× | 4.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.165 | 1.72× |
| 1 | 5 | 0.312 | 0.526 | 1.69× |
| 1 | 10 | 0.466 | 1.028 | 2.20× |
| 10 | 1 | 0.046 | 0.100 | 2.16× |
| 10 | 5 | 0.217 | 0.482 | 2.23× |
| 10 | 10 | 0.458 | 1.027 | 2.24× |
| 100 | 1 | 0.047 | 0.105 | 2.23× |
| 100 | 5 | 0.213 | 0.502 | 2.36× |
| 100 | 10 | 0.483 | 1.049 | 2.17× |
| 1,000 | 1 | 0.052 | 0.114 | 2.20× |
| 1,000 | 5 | 0.230 | 0.551 | 2.39× |
| 1,000 | 10 | 0.494 | 1.145 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
