# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.72M | 0.007 | 151.50M | 0.413 | 36.21× | 62.54× |
| 10,000 | 0.060 | 167.48M | 0.054 | 185.53M | 2.406 | 40.30× | 44.65× |
| 100,000 | 0.554 | 180.46M | 0.493 | 202.67M | 22.741 | 41.04× | 46.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.332 | 2.84× |
| 1 | 5 | 0.350 | 1.541 | 4.40× |
| 1 | 10 | 0.405 | 2.699 | 6.66× |
| 10 | 1 | 0.049 | 0.251 | 5.13× |
| 10 | 5 | 0.192 | 1.452 | 7.55× |
| 10 | 10 | 0.416 | 2.811 | 6.75× |
| 100 | 1 | 0.047 | 0.273 | 5.78× |
| 100 | 5 | 0.197 | 1.528 | 7.77× |
| 100 | 10 | 0.451 | 2.847 | 6.32× |
| 1,000 | 1 | 0.051 | 0.480 | 9.49× |
| 1,000 | 5 | 0.233 | 2.722 | 11.67× |
| 1,000 | 10 | 0.433 | 5.101 | 11.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
