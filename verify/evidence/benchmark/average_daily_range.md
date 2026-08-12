# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.83M | 0.014 | 71.68M | 0.423 | 24.47× | 30.34× |
| 10,000 | 0.072 | 138.69M | 0.066 | 151.52M | 2.493 | 34.58× | 37.78× |
| 100,000 | 0.597 | 167.38M | 0.563 | 177.48M | 23.295 | 38.99× | 41.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.351 | 3.51× |
| 1 | 5 | 0.357 | 1.360 | 3.81× |
| 1 | 10 | 0.551 | 2.787 | 5.06× |
| 10 | 1 | 0.059 | 0.272 | 4.58× |
| 10 | 5 | 0.279 | 1.540 | 5.52× |
| 10 | 10 | 0.597 | 3.005 | 5.03× |
| 100 | 1 | 0.063 | 0.294 | 4.64× |
| 100 | 5 | 0.266 | 1.664 | 6.25× |
| 100 | 10 | 0.603 | 3.151 | 5.22× |
| 1,000 | 1 | 0.067 | 0.502 | 7.45× |
| 1,000 | 5 | 0.296 | 2.748 | 9.30× |
| 1,000 | 10 | 0.602 | 5.326 | 8.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
