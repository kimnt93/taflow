# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 280.52M | 0.002 | 404.70M | 0.033 | 9.35× | 13.48× |
| 10,000 | 0.021 | 483.90M | 0.017 | 574.82M | 0.044 | 2.14× | 2.54× |
| 100,000 | 0.187 | 534.98M | 0.176 | 568.08M | 0.130 | 0.70× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.127 | 1.05× |
| 1 | 5 | 0.251 | 0.539 | 2.15× |
| 1 | 10 | 0.434 | 0.906 | 2.09× |
| 10 | 1 | 0.049 | 0.093 | 1.92× |
| 10 | 5 | 0.202 | 0.462 | 2.29× |
| 10 | 10 | 0.456 | 1.009 | 2.21× |
| 100 | 1 | 0.049 | 0.104 | 2.14× |
| 100 | 5 | 0.212 | 0.498 | 2.35× |
| 100 | 10 | 0.401 | 1.008 | 2.51× |
| 1,000 | 1 | 0.045 | 0.097 | 2.16× |
| 1,000 | 5 | 0.204 | 0.492 | 2.41× |
| 1,000 | 10 | 0.427 | 0.980 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
