# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.104 | 9.64M | 0.092 | 10.92M | 0.632 | 6.09× | 6.91× |
| 10,000 | 0.807 | 12.39M | 0.816 | 12.25M | 4.944 | 6.13× | 6.06× |
| 100,000 | 7.758 | 12.89M | 7.459 | 13.41M | 51.994 | 6.70× | 6.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.276 | 2.25× |
| 1 | 5 | 0.525 | 1.166 | 2.22× |
| 1 | 10 | 0.694 | 2.352 | 3.39× |
| 10 | 1 | 0.083 | 0.233 | 2.82× |
| 10 | 5 | 0.334 | 1.290 | 3.87× |
| 10 | 10 | 0.702 | 2.440 | 3.47× |
| 100 | 1 | 0.096 | 0.270 | 2.83× |
| 100 | 5 | 0.334 | 1.534 | 4.59× |
| 100 | 10 | 0.702 | 2.850 | 4.06× |
| 1,000 | 1 | 0.160 | 0.901 | 5.63× |
| 1,000 | 5 | 0.364 | 4.006 | 11.02× |
| 1,000 | 10 | 0.737 | 9.773 | 13.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
