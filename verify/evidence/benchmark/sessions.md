# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.38M | 0.058 | 17.22M | 84.710 | 1218.15× | 1458.72× |
| 10,000 | 0.505 | 19.79M | 0.457 | 21.87M | 838.423 | 1659.13× | 1833.44× |
| 100,000 | 4.647 | 21.52M | 4.512 | 22.16M | 8328.693 | 1792.25× | 1845.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 1.667 | 13.85× |
| 1 | 5 | 0.362 | 8.406 | 23.21× |
| 1 | 10 | 0.613 | 16.442 | 26.81× |
| 10 | 1 | 0.084 | 2.428 | 28.91× |
| 10 | 5 | 0.314 | 12.635 | 40.21× |
| 10 | 10 | 0.596 | 24.975 | 41.91× |
| 100 | 1 | 0.094 | 10.514 | 111.73× |
| 100 | 5 | 0.391 | 57.177 | 146.15× |
| 100 | 10 | 0.997 | 121.583 | 121.96× |
| 1,000 | 1 | 0.194 | 87.618 | 451.59× |
| 1,000 | 5 | 0.543 | 577.808 | 1064.99× |
| 1,000 | 10 | 0.807 | 1113.619 | 1380.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
