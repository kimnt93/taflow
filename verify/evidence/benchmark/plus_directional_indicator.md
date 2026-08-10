# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.44M | 0.011 | 91.44M | 0.039 | 2.91× | 3.53× |
| 10,000 | 0.069 | 144.27M | 0.064 | 155.85M | 0.097 | 1.41× | 1.52× |
| 100,000 | 0.582 | 171.73M | 0.550 | 181.79M | 0.656 | 1.13× | 1.19× |
| 1,000,000 | 6.233 | 160.44M | 5.655 | 176.83M | 6.632 | 1.06× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.140 | 1.16× |
| 1 | 5 | 0.352 | 0.496 | 1.41× |
| 1 | 10 | 0.514 | 0.961 | 1.87× |
| 10 | 1 | 0.051 | 0.095 | 1.85× |
| 10 | 5 | 0.233 | 0.450 | 1.93× |
| 10 | 10 | 0.514 | 0.943 | 1.83× |
| 100 | 1 | 0.056 | 0.094 | 1.68× |
| 100 | 5 | 0.233 | 0.458 | 1.96× |
| 100 | 10 | 0.510 | 0.947 | 1.86× |
| 1,000 | 1 | 0.061 | 0.100 | 1.65× |
| 1,000 | 5 | 0.253 | 0.481 | 1.90× |
| 1,000 | 10 | 0.544 | 1.021 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
