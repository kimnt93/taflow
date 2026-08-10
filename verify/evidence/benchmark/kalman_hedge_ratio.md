# KalmanHedgeRatio benchmark (`KalmanHedgeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.79M | 0.017 | 60.59M | 0.539 | 29.51× | 32.64× |
| 10,000 | 0.137 | 73.01M | 0.128 | 78.09M | 3.569 | 26.06× | 27.87× |
| 100,000 | 1.273 | 78.54M | 1.234 | 81.04M | 40.383 | 31.72× | 32.73× |
| 1,000,000 | 13.126 | 76.18M | 12.471 | 80.18M | 445.983 | 33.98× | 35.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.323 | 3.82× |
| 1 | 5 | 0.304 | 1.479 | 4.87× |
| 1 | 10 | 0.488 | 2.859 | 5.86× |
| 10 | 1 | 0.054 | 0.262 | 4.89× |
| 10 | 5 | 0.241 | 1.453 | 6.03× |
| 10 | 10 | 0.484 | 2.867 | 5.93× |
| 100 | 1 | 0.055 | 0.295 | 5.37× |
| 100 | 5 | 0.240 | 1.669 | 6.95× |
| 100 | 10 | 0.556 | 3.265 | 5.87× |
| 1,000 | 1 | 0.072 | 0.905 | 12.66× |
| 1,000 | 5 | 0.234 | 3.609 | 15.41× |
| 1,000 | 10 | 0.516 | 7.304 | 14.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
