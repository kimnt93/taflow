# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.82M | 0.039 | 25.57M | 0.895 | 20.44× | 22.89× |
| 10,000 | 0.320 | 31.23M | 0.326 | 30.70M | 7.476 | 23.35× | 22.95× |
| 100,000 | 3.593 | 27.83M | 3.150 | 31.75M | 77.953 | 21.69× | 24.75× |
| 1,000,000 | 65.267 | 15.32M | 46.369 | 21.57M | 915.086 | 14.02× | 19.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.293 | 3.34× |
| 1 | 5 | 0.302 | 1.159 | 3.84× |
| 1 | 10 | 0.607 | 2.565 | 4.23× |
| 10 | 1 | 0.062 | 0.250 | 4.02× |
| 10 | 5 | 0.310 | 1.470 | 4.75× |
| 10 | 10 | 0.605 | 2.614 | 4.32× |
| 100 | 1 | 0.064 | 0.330 | 5.13× |
| 100 | 5 | 0.331 | 1.792 | 5.41× |
| 100 | 10 | 0.580 | 3.305 | 5.70× |
| 1,000 | 1 | 0.097 | 1.180 | 12.12× |
| 1,000 | 5 | 0.297 | 5.945 | 20.01× |
| 1,000 | 10 | 0.677 | 11.524 | 17.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
