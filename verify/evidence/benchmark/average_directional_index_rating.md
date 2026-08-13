# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.082 | 12.13M | 0.070 | 14.25M | 0.045 | 0.54× | 0.64× |
| 10,000 | 0.699 | 14.30M | 0.608 | 16.44M | 0.122 | 0.18× | 0.20× |
| 100,000 | 5.846 | 17.11M | 5.772 | 17.32M | 0.989 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.127 | 1.11× |
| 1 | 5 | 0.416 | 0.542 | 1.30× |
| 1 | 10 | 0.649 | 0.942 | 1.45× |
| 10 | 1 | 0.072 | 0.095 | 1.31× |
| 10 | 5 | 0.309 | 0.445 | 1.44× |
| 10 | 10 | 0.637 | 0.923 | 1.45× |
| 100 | 1 | 0.077 | 0.089 | 1.16× |
| 100 | 5 | 0.324 | 0.456 | 1.41× |
| 100 | 10 | 0.707 | 0.982 | 1.39× |
| 1,000 | 1 | 0.131 | 0.101 | 0.77× |
| 1,000 | 5 | 0.327 | 0.511 | 1.56× |
| 1,000 | 10 | 0.664 | 1.042 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
