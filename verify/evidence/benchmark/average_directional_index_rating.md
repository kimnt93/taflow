# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.11M | 0.018 | 56.27M | 0.044 | 2.23× | 2.46× |
| 10,000 | 0.129 | 77.56M | 0.119 | 83.79M | 0.129 | 1.00× | 1.08× |
| 100,000 | 1.197 | 83.52M | 1.135 | 88.10M | 1.350 | 1.13× | 1.19× |
| 1,000,000 | 12.272 | 81.49M | 12.368 | 80.85M | 11.030 | 0.90× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.099 | 1.21× |
| 1 | 5 | 0.348 | 0.545 | 1.57× |
| 1 | 10 | 0.597 | 1.130 | 1.89× |
| 10 | 1 | 0.062 | 0.091 | 1.46× |
| 10 | 5 | 0.259 | 0.491 | 1.90× |
| 10 | 10 | 0.594 | 1.069 | 1.80× |
| 100 | 1 | 0.072 | 0.114 | 1.59× |
| 100 | 5 | 0.311 | 0.524 | 1.68× |
| 100 | 10 | 0.595 | 1.071 | 1.80× |
| 1,000 | 1 | 0.071 | 0.118 | 1.65× |
| 1,000 | 5 | 0.350 | 0.619 | 1.77× |
| 1,000 | 10 | 0.652 | 1.206 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
