# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.26M | 0.010 | 96.23M | 0.054 | 4.60× | 5.20× |
| 10,000 | 0.060 | 167.97M | 0.055 | 182.87M | 0.095 | 1.59× | 1.73× |
| 100,000 | 0.524 | 190.81M | 0.517 | 193.29M | 0.531 | 1.01× | 1.03× |
| 1,000,000 | 5.945 | 168.20M | 5.375 | 186.06M | 5.220 | 0.88× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.147 | 1.24× |
| 1 | 5 | 0.384 | 0.605 | 1.57× |
| 1 | 10 | 0.531 | 1.001 | 1.88× |
| 10 | 1 | 0.050 | 0.093 | 1.86× |
| 10 | 5 | 0.229 | 0.462 | 2.01× |
| 10 | 10 | 0.503 | 1.073 | 2.13× |
| 100 | 1 | 0.051 | 0.096 | 1.87× |
| 100 | 5 | 0.234 | 0.469 | 2.00× |
| 100 | 10 | 0.499 | 1.070 | 2.15× |
| 1,000 | 1 | 0.057 | 0.103 | 1.81× |
| 1,000 | 5 | 0.256 | 0.502 | 1.96× |
| 1,000 | 10 | 0.510 | 1.078 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
