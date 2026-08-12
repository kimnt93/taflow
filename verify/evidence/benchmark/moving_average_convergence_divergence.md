# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.30M | 0.005 | 200.10M | 0.055 | 8.18× | 11.04× |
| 10,000 | 0.034 | 292.46M | 0.026 | 383.57M | 0.145 | 4.24× | 5.56× |
| 100,000 | 1.283 | 77.94M | 0.228 | 439.21M | 1.695 | 1.32× | 7.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.131 | 1.53× |
| 1 | 5 | 0.295 | 0.528 | 1.79× |
| 1 | 10 | 0.464 | 1.136 | 2.45× |
| 10 | 1 | 0.051 | 0.103 | 2.03× |
| 10 | 5 | 0.235 | 0.505 | 2.15× |
| 10 | 10 | 0.496 | 1.095 | 2.21× |
| 100 | 1 | 0.062 | 0.115 | 1.86× |
| 100 | 5 | 0.250 | 0.504 | 2.01× |
| 100 | 10 | 0.460 | 1.051 | 2.29× |
| 1,000 | 1 | 0.053 | 0.117 | 2.22× |
| 1,000 | 5 | 0.257 | 0.629 | 2.45× |
| 1,000 | 10 | 0.525 | 1.169 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
