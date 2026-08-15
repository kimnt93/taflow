# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.18M | 0.038 | 26.44M | 0.637 | 15.41× | 16.86× |
| 10,000 | 0.386 | 25.91M | 0.374 | 26.76M | 4.353 | 11.28× | 11.65× |
| 100,000 | 3.847 | 25.99M | 3.673 | 27.23M | 48.849 | 12.70× | 13.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.346 | 4.43× |
| 1 | 5 | 0.279 | 1.462 | 5.24× |
| 1 | 10 | 0.406 | 2.705 | 6.65× |
| 10 | 1 | 0.048 | 0.250 | 5.16× |
| 10 | 5 | 0.192 | 1.465 | 7.64× |
| 10 | 10 | 0.464 | 2.858 | 6.16× |
| 100 | 1 | 0.054 | 0.296 | 5.51× |
| 100 | 5 | 0.196 | 1.732 | 8.82× |
| 100 | 10 | 0.439 | 3.104 | 7.07× |
| 1,000 | 1 | 0.122 | 0.947 | 7.79× |
| 1,000 | 5 | 0.201 | 3.857 | 19.19× |
| 1,000 | 10 | 0.489 | 7.838 | 16.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
