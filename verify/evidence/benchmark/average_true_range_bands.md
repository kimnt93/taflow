# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.59M | 0.014 | 70.48M | 0.606 | 34.32× | 42.74× |
| 10,000 | 0.096 | 104.54M | 0.086 | 115.78M | 4.338 | 45.35× | 50.23× |
| 100,000 | 0.948 | 105.48M | 0.797 | 125.54M | 44.031 | 46.44× | 55.28× |
| 1,000,000 | 10.513 | 95.12M | 8.430 | 118.62M | 504.979 | 48.03× | 59.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.339 | 2.60× |
| 1 | 5 | 0.276 | 1.486 | 5.39× |
| 1 | 10 | 0.516 | 2.761 | 5.35× |
| 10 | 1 | 0.061 | 0.277 | 4.58× |
| 10 | 5 | 0.256 | 1.442 | 5.64× |
| 10 | 10 | 0.562 | 2.891 | 5.14× |
| 100 | 1 | 0.059 | 0.307 | 5.17× |
| 100 | 5 | 0.251 | 1.804 | 7.19× |
| 100 | 10 | 0.559 | 3.263 | 5.83× |
| 1,000 | 1 | 0.071 | 0.933 | 13.11× |
| 1,000 | 5 | 0.254 | 3.834 | 15.10× |
| 1,000 | 10 | 0.602 | 8.464 | 14.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
