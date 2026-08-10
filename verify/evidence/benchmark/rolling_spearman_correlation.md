# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.418 | 2.39M | 0.390 | 2.56M | 0.789 | 1.89× | 2.02× |
| 10,000 | 4.238 | 2.36M | 4.071 | 2.46M | 6.571 | 1.55× | 1.61× |
| 100,000 | 40.154 | 2.49M | 40.807 | 2.45M | 65.086 | 1.62× | 1.59× |
| 1,000,000 | 413.350 | 2.42M | 408.790 | 2.45M | 637.102 | 1.54× | 1.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.311 | 2.62× |
| 1 | 5 | 0.416 | 1.225 | 2.94× |
| 1 | 10 | 0.467 | 2.483 | 5.32× |
| 10 | 1 | 0.052 | 0.212 | 4.07× |
| 10 | 5 | 0.236 | 1.389 | 5.88× |
| 10 | 10 | 0.551 | 2.421 | 4.40× |
| 100 | 1 | 0.090 | 0.263 | 2.92× |
| 100 | 5 | 0.286 | 1.739 | 6.08× |
| 100 | 10 | 0.554 | 3.005 | 5.42× |
| 1,000 | 1 | 0.466 | 0.881 | 1.89× |
| 1,000 | 5 | 0.737 | 4.655 | 6.32× |
| 1,000 | 10 | 1.263 | 9.133 | 7.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
