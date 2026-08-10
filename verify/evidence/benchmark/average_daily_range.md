# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.84M | 0.015 | 67.04M | 0.453 | 23.93× | 30.36× |
| 10,000 | 0.069 | 144.19M | 0.066 | 152.55M | 2.512 | 36.22× | 38.33× |
| 100,000 | 0.563 | 177.77M | 0.543 | 184.02M | 23.645 | 42.03× | 43.51× |
| 1,000,000 | 6.526 | 153.24M | 6.072 | 164.68M | 270.632 | 41.47× | 44.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.459 | 5.00× |
| 1 | 5 | 0.433 | 1.343 | 3.10× |
| 1 | 10 | 0.604 | 2.864 | 4.75× |
| 10 | 1 | 0.066 | 0.281 | 4.23× |
| 10 | 5 | 0.290 | 1.552 | 5.35× |
| 10 | 10 | 0.579 | 3.035 | 5.24× |
| 100 | 1 | 0.064 | 0.290 | 4.52× |
| 100 | 5 | 0.293 | 1.646 | 5.62× |
| 100 | 10 | 0.617 | 3.107 | 5.03× |
| 1,000 | 1 | 0.081 | 0.516 | 6.41× |
| 1,000 | 5 | 0.314 | 2.744 | 8.75× |
| 1,000 | 10 | 0.603 | 5.485 | 9.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
