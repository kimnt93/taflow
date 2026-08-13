# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.091 | 10.97M | 0.069 | 14.45M | 0.566 | 6.21× | 8.17× |
| 10,000 | 0.664 | 15.06M | 0.657 | 15.22M | 4.409 | 6.64× | 6.71× |
| 100,000 | 6.061 | 16.50M | 5.753 | 17.38M | 49.551 | 8.17× | 8.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.293 | 3.30× |
| 1 | 5 | 0.326 | 1.412 | 4.33× |
| 1 | 10 | 0.664 | 2.853 | 4.30× |
| 10 | 1 | 0.079 | 0.264 | 3.33× |
| 10 | 5 | 0.319 | 1.440 | 4.51× |
| 10 | 10 | 0.664 | 2.792 | 4.20× |
| 100 | 1 | 0.083 | 0.295 | 3.54× |
| 100 | 5 | 0.329 | 1.649 | 5.01× |
| 100 | 10 | 0.691 | 3.116 | 4.51× |
| 1,000 | 1 | 0.142 | 0.949 | 6.70× |
| 1,000 | 5 | 0.350 | 3.743 | 10.69× |
| 1,000 | 10 | 0.725 | 7.521 | 10.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
