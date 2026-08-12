# KalmanHedgeRatio benchmark (`KalmanHedgeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.18M | 0.016 | 62.79M | 0.536 | 30.64× | 33.65× |
| 10,000 | 0.158 | 63.22M | 0.134 | 74.48M | 3.730 | 23.58× | 27.78× |
| 100,000 | 1.294 | 77.25M | 1.271 | 78.69M | 41.785 | 32.28× | 32.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.310 | 3.71× |
| 1 | 5 | 0.275 | 1.400 | 5.09× |
| 1 | 10 | 0.467 | 2.831 | 6.06× |
| 10 | 1 | 0.056 | 0.261 | 4.62× |
| 10 | 5 | 0.235 | 1.482 | 6.32× |
| 10 | 10 | 0.478 | 2.953 | 6.17× |
| 100 | 1 | 0.055 | 0.299 | 5.48× |
| 100 | 5 | 0.239 | 1.679 | 7.01× |
| 100 | 10 | 0.502 | 3.234 | 6.44× |
| 1,000 | 1 | 0.065 | 0.921 | 14.08× |
| 1,000 | 5 | 0.242 | 3.629 | 15.02× |
| 1,000 | 10 | 0.531 | 7.293 | 13.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
