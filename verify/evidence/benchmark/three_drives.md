# ThreeDrives benchmark (`ThreeDrives` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.63M | 0.012 | 80.03M | 0.227 | 14.25× | 18.20× |
| 10,000 | 0.112 | 89.43M | 0.106 | 94.74M | 1.489 | 13.32× | 14.11× |
| 100,000 | 0.981 | 101.93M | 0.941 | 106.29M | 13.261 | 13.52× | 14.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.216 | 2.31× |
| 1 | 5 | 0.326 | 0.859 | 2.64× |
| 1 | 10 | 0.551 | 1.883 | 3.42× |
| 10 | 1 | 0.063 | 0.168 | 2.67× |
| 10 | 5 | 0.246 | 1.108 | 4.51× |
| 10 | 10 | 0.570 | 1.744 | 3.06× |
| 100 | 1 | 0.059 | 0.183 | 3.10× |
| 100 | 5 | 0.263 | 1.194 | 4.55× |
| 100 | 10 | 0.591 | 1.866 | 3.16× |
| 1,000 | 1 | 0.066 | 0.306 | 4.63× |
| 1,000 | 5 | 0.279 | 1.854 | 6.65× |
| 1,000 | 10 | 0.575 | 3.061 | 5.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
