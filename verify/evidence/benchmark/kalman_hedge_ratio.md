# KalmanHedgeRatio benchmark (`KalmanHedgeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.78M | 0.016 | 61.89M | 0.569 | 32.90× | 35.24× |
| 10,000 | 0.147 | 67.93M | 0.144 | 69.48M | 3.874 | 26.32× | 26.92× |
| 100,000 | 1.465 | 68.24M | 1.418 | 70.51M | 43.992 | 30.02× | 31.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.327 | 5.02× |
| 1 | 5 | 0.290 | 1.439 | 4.97× |
| 1 | 10 | 0.396 | 2.765 | 6.97× |
| 10 | 1 | 0.048 | 0.259 | 5.45× |
| 10 | 5 | 0.182 | 1.526 | 8.40× |
| 10 | 10 | 0.423 | 3.023 | 7.14× |
| 100 | 1 | 0.046 | 0.305 | 6.63× |
| 100 | 5 | 0.207 | 1.866 | 9.03× |
| 100 | 10 | 0.425 | 3.170 | 7.46× |
| 1,000 | 1 | 0.068 | 0.927 | 13.61× |
| 1,000 | 5 | 0.203 | 3.965 | 19.52× |
| 1,000 | 10 | 0.434 | 7.489 | 17.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
