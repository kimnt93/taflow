# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 294.10M | 0.001 | 723.51M | 0.033 | 9.58× | 23.57× |
| 10,000 | 0.010 | 959.73M | 0.006 | 1.54G | 0.036 | 3.48× | 5.60× |
| 100,000 | 0.084 | 1.19G | 0.058 | 1.73G | 0.108 | 1.29× | 1.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.172 | 1.38× |
| 1 | 5 | 0.232 | 0.524 | 2.26× |
| 1 | 10 | 0.476 | 1.042 | 2.19× |
| 10 | 1 | 0.046 | 0.087 | 1.86× |
| 10 | 5 | 0.205 | 0.442 | 2.16× |
| 10 | 10 | 0.412 | 1.081 | 2.62× |
| 100 | 1 | 0.050 | 0.101 | 2.01× |
| 100 | 5 | 0.236 | 0.493 | 2.08× |
| 100 | 10 | 0.525 | 1.033 | 1.97× |
| 1,000 | 1 | 0.051 | 0.090 | 1.78× |
| 1,000 | 5 | 0.226 | 0.524 | 2.32× |
| 1,000 | 10 | 0.488 | 1.037 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
