# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.31M | 0.011 | 89.04M | 0.596 | 40.13× | 53.08× |
| 10,000 | 0.104 | 96.58M | 0.091 | 110.11M | 4.082 | 39.43× | 44.95× |
| 100,000 | 0.986 | 101.41M | 0.904 | 110.56M | 46.849 | 47.51× | 51.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.278 | 2.71× |
| 1 | 5 | 0.344 | 1.430 | 4.16× |
| 1 | 10 | 0.385 | 2.632 | 6.83× |
| 10 | 1 | 0.053 | 0.267 | 5.01× |
| 10 | 5 | 0.196 | 1.471 | 7.49× |
| 10 | 10 | 0.425 | 2.919 | 6.87× |
| 100 | 1 | 0.050 | 0.306 | 6.06× |
| 100 | 5 | 0.208 | 1.626 | 7.82× |
| 100 | 10 | 0.478 | 3.059 | 6.40× |
| 1,000 | 1 | 0.057 | 0.883 | 15.42× |
| 1,000 | 5 | 0.229 | 3.719 | 16.27× |
| 1,000 | 10 | 0.440 | 14.656 | 33.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
