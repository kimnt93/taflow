# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.670 | 1.49M | 0.661 | 1.51M | 1.183 | 1.77× | 1.79× |
| 10,000 | 7.083 | 1.41M | 6.606 | 1.51M | 7.484 | 1.06× | 1.13× |
| 100,000 | 68.555 | 1.46M | 67.664 | 1.48M | 75.294 | 1.10× | 1.11× |
| 1,000,000 | 679.814 | 1.47M | 678.536 | 1.47M | 706.674 | 1.04× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.351 | 2.18× |
| 1 | 5 | 0.257 | 1.294 | 5.04× |
| 1 | 10 | 0.468 | 2.298 | 4.91× |
| 10 | 1 | 0.073 | 0.243 | 3.32× |
| 10 | 5 | 0.254 | 1.283 | 5.04× |
| 10 | 10 | 0.551 | 2.415 | 4.39× |
| 100 | 1 | 0.122 | 0.273 | 2.25× |
| 100 | 5 | 0.267 | 1.537 | 5.77× |
| 100 | 10 | 0.612 | 2.980 | 4.87× |
| 1,000 | 1 | 0.730 | 0.963 | 1.32× |
| 1,000 | 5 | 0.969 | 4.998 | 5.16× |
| 1,000 | 10 | 1.382 | 9.777 | 7.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
