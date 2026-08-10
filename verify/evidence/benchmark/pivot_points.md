# PivotPoints benchmark (`anchored classic pivot points` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.20M | 0.014 | 70.87M | 0.863 | 46.75× | 61.13× |
| 10,000 | 0.100 | 99.80M | 0.087 | 114.50M | 8.643 | 86.25× | 98.96× |
| 100,000 | 0.991 | 100.93M | 0.833 | 120.08M | 85.111 | 85.90× | 102.20× |
| 1,000,000 | 29.672 | 33.70M | 10.191 | 98.12M | 904.272 | 30.48× | 88.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.218 | 1.66× |
| 1 | 5 | 0.357 | 0.341 | 0.96× |
| 1 | 10 | 0.441 | 0.691 | 1.56× |
| 10 | 1 | 0.057 | 0.093 | 1.64× |
| 10 | 5 | 0.220 | 0.400 | 1.82× |
| 10 | 10 | 0.454 | 0.826 | 1.82× |
| 100 | 1 | 0.061 | 0.171 | 2.82× |
| 100 | 5 | 0.236 | 0.824 | 3.49× |
| 100 | 10 | 0.492 | 1.666 | 3.39× |
| 1,000 | 1 | 0.065 | 1.009 | 15.46× |
| 1,000 | 5 | 0.289 | 4.987 | 17.28× |
| 1,000 | 10 | 0.589 | 10.386 | 17.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
