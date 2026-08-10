# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.60M | 0.011 | 90.38M | 0.073 | 7.68× | 6.58× |
| 10,000 | 0.083 | 120.18M | 0.049 | 203.65M | 0.115 | 1.39× | 2.35× |
| 100,000 | 0.494 | 202.55M | 0.398 | 251.17M | 0.671 | 1.36× | 1.69× |
| 1,000,000 | 18.098 | 55.25M | 5.427 | 184.26M | 7.413 | 0.41× | 1.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.206 | 2.13× |
| 1 | 5 | 0.334 | 0.770 | 2.30× |
| 1 | 10 | 0.610 | 1.447 | 2.37× |
| 10 | 1 | 0.072 | 0.145 | 2.02× |
| 10 | 5 | 0.301 | 0.651 | 2.16× |
| 10 | 10 | 0.577 | 1.529 | 2.65× |
| 100 | 1 | 0.065 | 0.119 | 1.83× |
| 100 | 5 | 0.294 | 0.700 | 2.38× |
| 100 | 10 | 0.629 | 1.493 | 2.37× |
| 1,000 | 1 | 0.069 | 0.134 | 1.95× |
| 1,000 | 5 | 0.321 | 0.612 | 1.90× |
| 1,000 | 10 | 0.553 | 1.404 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
