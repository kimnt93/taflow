# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.92M | 0.010 | 98.32M | 0.593 | 40.27× | 58.30× |
| 10,000 | 0.095 | 104.72M | 0.082 | 122.02M | 4.156 | 43.52× | 50.71× |
| 100,000 | 0.936 | 106.89M | 0.796 | 125.63M | 46.826 | 50.05× | 58.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.313 | 4.52× |
| 1 | 5 | 0.241 | 1.337 | 5.54× |
| 1 | 10 | 0.419 | 2.620 | 6.26× |
| 10 | 1 | 0.052 | 0.252 | 4.85× |
| 10 | 5 | 0.194 | 1.447 | 7.46× |
| 10 | 10 | 0.417 | 2.807 | 6.73× |
| 100 | 1 | 0.046 | 0.297 | 6.49× |
| 100 | 5 | 0.233 | 1.667 | 7.14× |
| 100 | 10 | 0.432 | 3.166 | 7.32× |
| 1,000 | 1 | 0.060 | 0.859 | 14.42× |
| 1,000 | 5 | 0.206 | 3.677 | 17.84× |
| 1,000 | 10 | 0.427 | 7.485 | 17.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
