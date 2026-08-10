# RollingVolumeWeightedAveragePrice benchmark (`RollingVWAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.76M | 0.028 | 36.02M | 0.248 | 7.62× | 8.92× |
| 10,000 | 0.216 | 46.22M | 0.223 | 44.94M | 1.320 | 6.10× | 5.93× |
| 100,000 | 3.814 | 26.22M | 2.452 | 40.79M | 11.792 | 3.09× | 4.81× |
| 1,000,000 | 21.320 | 46.91M | 20.548 | 48.67M | 137.376 | 6.44× | 6.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.252 | 2.95× |
| 1 | 5 | 0.292 | 1.472 | 5.04× |
| 1 | 10 | 0.539 | 2.471 | 4.58× |
| 10 | 1 | 0.059 | 0.202 | 3.42× |
| 10 | 5 | 0.259 | 1.052 | 4.06× |
| 10 | 10 | 0.559 | 2.485 | 4.44× |
| 100 | 1 | 0.068 | 0.223 | 3.30× |
| 100 | 5 | 0.294 | 1.183 | 4.02× |
| 100 | 10 | 0.582 | 2.586 | 4.44× |
| 1,000 | 1 | 0.086 | 0.341 | 3.96× |
| 1,000 | 5 | 0.297 | 1.711 | 5.76× |
| 1,000 | 10 | 0.593 | 4.286 | 7.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
