# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.85M | 0.005 | 199.75M | 0.054 | 8.04× | 10.86× |
| 10,000 | 0.047 | 213.65M | 0.039 | 256.37M | 0.144 | 3.08× | 3.69× |
| 100,000 | 1.347 | 74.23M | 0.381 | 262.59M | 1.708 | 1.27× | 4.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.136 | 2.00× |
| 1 | 5 | 0.304 | 0.563 | 1.85× |
| 1 | 10 | 0.396 | 1.135 | 2.87× |
| 10 | 1 | 0.048 | 0.116 | 2.40× |
| 10 | 5 | 0.181 | 0.508 | 2.80× |
| 10 | 10 | 0.405 | 1.039 | 2.57× |
| 100 | 1 | 0.044 | 0.118 | 2.70× |
| 100 | 5 | 0.192 | 0.557 | 2.91× |
| 100 | 10 | 0.407 | 1.064 | 2.61× |
| 1,000 | 1 | 0.049 | 0.117 | 2.41× |
| 1,000 | 5 | 0.189 | 0.557 | 2.95× |
| 1,000 | 10 | 0.467 | 1.184 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
