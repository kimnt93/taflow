# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.89M | 0.009 | 107.22M | 0.037 | 2.91× | 3.96× |
| 10,000 | 0.106 | 94.31M | 0.103 | 96.81M | 0.131 | 1.23× | 1.27× |
| 100,000 | 1.027 | 97.37M | 1.230 | 81.31M | 1.113 | 1.08× | 0.90× |
| 1,000,000 | 10.387 | 96.27M | 10.320 | 96.90M | 11.817 | 1.14× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.137 | 0.78× |
| 1 | 5 | 0.323 | 0.451 | 1.40× |
| 1 | 10 | 0.542 | 0.876 | 1.61× |
| 10 | 1 | 0.051 | 0.090 | 1.75× |
| 10 | 5 | 0.249 | 0.418 | 1.68× |
| 10 | 10 | 0.530 | 0.900 | 1.70× |
| 100 | 1 | 0.060 | 0.094 | 1.56× |
| 100 | 5 | 0.265 | 0.427 | 1.61× |
| 100 | 10 | 0.525 | 0.899 | 1.71× |
| 1,000 | 1 | 0.065 | 0.102 | 1.58× |
| 1,000 | 5 | 0.272 | 0.484 | 1.78× |
| 1,000 | 10 | 0.573 | 1.017 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
