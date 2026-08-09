# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.39M | 0.005 | 213.49M | 0.042 | 7.36× | 8.95× |
| 10,000 | 0.029 | 346.43M | 0.029 | 349.71M | 0.063 | 2.17× | 2.19× |
| 100,000 | 0.266 | 376.06M | 0.238 | 420.18M | 0.295 | 1.11× | 1.24× |
| 1,000,000 | 3.806 | 262.75M | 2.992 | 334.24M | 3.026 | 0.79× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.122 | 1.34× |
| 1 | 5 | 0.383 | 0.568 | 1.48× |
| 1 | 10 | 0.540 | 1.007 | 1.86× |
| 10 | 1 | 0.047 | 0.090 | 1.92× |
| 10 | 5 | 0.212 | 0.439 | 2.08× |
| 10 | 10 | 0.478 | 0.953 | 2.00× |
| 100 | 1 | 0.051 | 0.118 | 2.30× |
| 100 | 5 | 0.235 | 0.463 | 1.97× |
| 100 | 10 | 0.474 | 0.991 | 2.09× |
| 1,000 | 1 | 0.052 | 0.093 | 1.79× |
| 1,000 | 5 | 0.249 | 0.462 | 1.86× |
| 1,000 | 10 | 0.501 | 0.973 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
