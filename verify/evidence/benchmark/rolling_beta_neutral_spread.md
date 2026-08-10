# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.083 | 12.04M | 0.068 | 14.78M | 0.223 | 2.68× | 3.29× |
| 10,000 | 0.503 | 19.89M | 0.541 | 18.48M | 1.391 | 2.77× | 2.57× |
| 100,000 | 7.318 | 13.67M | 5.238 | 19.09M | 9.035 | 1.23× | 1.72× |
| 1,000,000 | 51.371 | 19.47M | 49.723 | 20.11M | 89.120 | 1.73× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.390 | 2.26× |
| 1 | 5 | 0.341 | 1.261 | 3.70× |
| 1 | 10 | 0.504 | 2.505 | 4.97× |
| 10 | 1 | 0.053 | 0.218 | 4.13× |
| 10 | 5 | 0.235 | 1.264 | 5.39× |
| 10 | 10 | 0.538 | 2.348 | 4.37× |
| 100 | 1 | 0.058 | 0.229 | 3.97× |
| 100 | 5 | 0.259 | 1.421 | 5.48× |
| 100 | 10 | 0.532 | 2.500 | 4.70× |
| 1,000 | 1 | 0.105 | 0.372 | 3.56× |
| 1,000 | 5 | 0.305 | 1.724 | 5.65× |
| 1,000 | 10 | 0.550 | 3.415 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
