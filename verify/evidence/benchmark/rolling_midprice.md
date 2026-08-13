# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.093 | 10.70M | 0.086 | 11.60M | 0.037 | 0.40× | 0.43× |
| 10,000 | 0.760 | 13.15M | 0.726 | 13.77M | 0.100 | 0.13× | 0.14× |
| 100,000 | 7.573 | 13.20M | 8.212 | 12.18M | 1.382 | 0.18× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.116 | 0.83× |
| 1 | 5 | 0.390 | 0.588 | 1.51× |
| 1 | 10 | 0.794 | 1.048 | 1.32× |
| 10 | 1 | 0.077 | 0.098 | 1.28× |
| 10 | 5 | 0.327 | 0.608 | 1.86× |
| 10 | 10 | 0.731 | 1.053 | 1.44× |
| 100 | 1 | 0.099 | 0.096 | 0.98× |
| 100 | 5 | 0.348 | 0.497 | 1.43× |
| 100 | 10 | 0.699 | 0.928 | 1.33× |
| 1,000 | 1 | 0.151 | 0.102 | 0.67× |
| 1,000 | 5 | 0.344 | 0.486 | 1.41× |
| 1,000 | 10 | 0.665 | 1.013 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
