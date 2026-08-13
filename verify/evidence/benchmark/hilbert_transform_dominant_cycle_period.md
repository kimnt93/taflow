# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.115 | 8.67M | 0.110 | 9.13M | 0.068 | 0.59× | 0.62× |
| 10,000 | 1.093 | 9.15M | 1.085 | 9.22M | 0.439 | 0.40× | 0.40× |
| 100,000 | 10.668 | 9.37M | 11.087 | 9.02M | 4.183 | 0.39× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.119 | 1.06× |
| 1 | 5 | 0.449 | 0.490 | 1.09× |
| 1 | 10 | 0.611 | 0.872 | 1.43× |
| 10 | 1 | 0.064 | 0.094 | 1.48× |
| 10 | 5 | 0.285 | 0.420 | 1.47× |
| 10 | 10 | 0.649 | 0.871 | 1.34× |
| 100 | 1 | 0.075 | 0.089 | 1.17× |
| 100 | 5 | 0.286 | 0.445 | 1.56× |
| 100 | 10 | 0.630 | 0.918 | 1.46× |
| 1,000 | 1 | 0.173 | 0.136 | 0.78× |
| 1,000 | 5 | 0.349 | 0.628 | 1.80× |
| 1,000 | 10 | 0.678 | 1.323 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
