# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.65M | 0.012 | 83.07M | 3.294 | 189.90× | 273.62× |
| 10,000 | 0.113 | 88.35M | 0.108 | 92.19M | 9.619 | 84.98× | 88.67× |
| 100,000 | 1.247 | 80.19M | 1.032 | 96.87M | 72.702 | 58.30× | 70.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 2.930 | 31.59× |
| 1 | 5 | 0.344 | 14.176 | 41.18× |
| 1 | 10 | 0.438 | 28.664 | 65.46× |
| 10 | 1 | 0.056 | 2.734 | 48.59× |
| 10 | 5 | 0.250 | 14.293 | 57.15× |
| 10 | 10 | 0.414 | 29.127 | 70.40× |
| 100 | 1 | 0.053 | 2.810 | 53.16× |
| 100 | 5 | 0.227 | 14.879 | 65.44× |
| 100 | 10 | 0.423 | 32.205 | 76.15× |
| 1,000 | 1 | 0.064 | 3.514 | 55.30× |
| 1,000 | 5 | 0.218 | 18.800 | 86.24× |
| 1,000 | 10 | 0.497 | 45.142 | 90.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
