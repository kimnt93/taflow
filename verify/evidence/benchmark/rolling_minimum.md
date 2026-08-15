# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 236.74M | 0.003 | 317.20M | 0.037 | 8.66× | 11.61× |
| 10,000 | 0.025 | 402.31M | 0.023 | 436.40M | 0.080 | 3.24× | 3.51× |
| 100,000 | 0.235 | 425.83M | 0.210 | 475.24M | 0.539 | 2.30× | 2.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.104 | 0.90× |
| 1 | 5 | 0.286 | 0.510 | 1.78× |
| 1 | 10 | 0.442 | 1.028 | 2.33× |
| 10 | 1 | 0.047 | 0.087 | 1.86× |
| 10 | 5 | 0.185 | 0.434 | 2.34× |
| 10 | 10 | 0.412 | 1.057 | 2.57× |
| 100 | 1 | 0.050 | 0.112 | 2.23× |
| 100 | 5 | 0.197 | 0.439 | 2.22× |
| 100 | 10 | 0.421 | 0.924 | 2.20× |
| 1,000 | 1 | 0.043 | 0.092 | 2.14× |
| 1,000 | 5 | 0.265 | 0.589 | 2.22× |
| 1,000 | 10 | 0.478 | 1.113 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
