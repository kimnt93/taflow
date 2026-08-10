# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.45M | 0.007 | 153.61M | 0.023 | 3.03× | 3.48× |
| 10,000 | 0.034 | 293.02M | 0.031 | 322.29M | 0.042 | 1.24× | 1.37× |
| 100,000 | 0.301 | 332.53M | 0.261 | 383.82M | 0.229 | 0.76× | 0.88× |
| 1,000,000 | 3.163 | 316.12M | 2.711 | 368.89M | 4.700 | 1.49× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.104 | 1.02× |
| 1 | 5 | 0.334 | 0.376 | 1.13× |
| 1 | 10 | 0.463 | 0.747 | 1.61× |
| 10 | 1 | 0.054 | 0.071 | 1.31× |
| 10 | 5 | 0.220 | 0.355 | 1.62× |
| 10 | 10 | 0.463 | 0.748 | 1.61× |
| 100 | 1 | 0.048 | 0.073 | 1.51× |
| 100 | 5 | 0.227 | 0.353 | 1.56× |
| 100 | 10 | 0.483 | 0.746 | 1.54× |
| 1,000 | 1 | 0.052 | 0.078 | 1.48× |
| 1,000 | 5 | 0.249 | 0.479 | 1.92× |
| 1,000 | 10 | 0.515 | 1.187 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
