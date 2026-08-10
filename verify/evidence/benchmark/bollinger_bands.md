# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.23M | 0.006 | 160.55M | 0.051 | 6.55× | 8.21× |
| 10,000 | 0.045 | 224.33M | 0.036 | 279.92M | 0.095 | 2.13× | 2.65× |
| 100,000 | 0.410 | 244.05M | 0.317 | 315.22M | 0.540 | 1.32× | 1.70× |
| 1,000,000 | 13.241 | 75.53M | 3.306 | 302.46M | 5.433 | 0.41× | 1.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.146 | 0.155 | 1.06× |
| 1 | 5 | 0.440 | 0.617 | 1.40× |
| 1 | 10 | 0.468 | 1.183 | 2.53× |
| 10 | 1 | 0.050 | 0.108 | 2.15× |
| 10 | 5 | 0.219 | 0.542 | 2.47× |
| 10 | 10 | 0.473 | 1.085 | 2.30× |
| 100 | 1 | 0.048 | 0.113 | 2.35× |
| 100 | 5 | 0.218 | 0.533 | 2.45× |
| 100 | 10 | 0.469 | 1.099 | 2.34× |
| 1,000 | 1 | 0.054 | 0.118 | 2.19× |
| 1,000 | 5 | 0.240 | 0.697 | 2.90× |
| 1,000 | 10 | 0.608 | 1.414 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
