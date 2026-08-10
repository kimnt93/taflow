# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.47M | 0.023 | 43.83M | 0.372 | 24.01× | 16.32× |
| 10,000 | 0.110 | 90.70M | 0.123 | 81.31M | 0.777 | 7.05× | 6.32× |
| 100,000 | 0.957 | 104.47M | 0.957 | 104.47M | 5.579 | 5.83× | 5.83× |
| 1,000,000 | 10.077 | 99.24M | 9.420 | 106.16M | 49.989 | 4.96× | 5.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.297 | 2.87× |
| 1 | 5 | 0.344 | 1.269 | 3.69× |
| 1 | 10 | 0.526 | 2.581 | 4.90× |
| 10 | 1 | 0.055 | 0.232 | 4.19× |
| 10 | 5 | 0.227 | 1.406 | 6.20× |
| 10 | 10 | 0.590 | 2.277 | 3.86× |
| 100 | 1 | 0.054 | 0.207 | 3.83× |
| 100 | 5 | 0.269 | 1.474 | 5.48× |
| 100 | 10 | 0.513 | 2.410 | 4.70× |
| 1,000 | 1 | 0.066 | 0.287 | 4.38× |
| 1,000 | 5 | 0.275 | 1.535 | 5.58× |
| 1,000 | 10 | 0.547 | 3.037 | 5.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
