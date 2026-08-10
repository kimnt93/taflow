# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.95M | 0.016 | 62.72M | 0.037 | 1.92× | 2.32× |
| 10,000 | 0.179 | 55.87M | 0.172 | 58.07M | 0.218 | 1.22× | 1.26× |
| 100,000 | 1.791 | 55.85M | 1.767 | 56.59M | 1.919 | 1.07× | 1.09× |
| 1,000,000 | 17.613 | 56.78M | 17.691 | 56.52M | 18.531 | 1.05× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.120 | 0.95× |
| 1 | 5 | 0.287 | 0.444 | 1.54× |
| 1 | 10 | 0.549 | 1.018 | 1.86× |
| 10 | 1 | 0.070 | 0.092 | 1.31× |
| 10 | 5 | 0.266 | 0.448 | 1.69× |
| 10 | 10 | 0.534 | 0.942 | 1.77× |
| 100 | 1 | 0.075 | 0.111 | 1.47× |
| 100 | 5 | 0.294 | 0.515 | 1.75× |
| 100 | 10 | 0.559 | 0.959 | 1.71× |
| 1,000 | 1 | 0.067 | 0.118 | 1.77× |
| 1,000 | 5 | 0.305 | 0.586 | 1.92× |
| 1,000 | 10 | 0.674 | 1.100 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
