# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.99M | 0.008 | 124.33M | 0.657 | 49.95× | 81.72× |
| 10,000 | 0.080 | 125.19M | 0.066 | 150.92M | 5.319 | 66.59× | 80.28× |
| 100,000 | 0.713 | 140.21M | 0.657 | 152.25M | 55.717 | 78.12× | 84.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.267 | 3.44× |
| 1 | 5 | 0.294 | 1.161 | 3.95× |
| 1 | 10 | 0.405 | 2.361 | 5.83× |
| 10 | 1 | 0.055 | 0.245 | 4.48× |
| 10 | 5 | 0.210 | 1.377 | 6.56× |
| 10 | 10 | 0.434 | 2.653 | 6.11× |
| 100 | 1 | 0.051 | 0.286 | 5.65× |
| 100 | 5 | 0.213 | 1.729 | 8.11× |
| 100 | 10 | 0.431 | 3.168 | 7.34× |
| 1,000 | 1 | 0.057 | 0.846 | 14.88× |
| 1,000 | 5 | 0.197 | 4.150 | 21.09× |
| 1,000 | 10 | 0.425 | 8.202 | 19.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
