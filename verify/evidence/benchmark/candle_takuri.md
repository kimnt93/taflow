# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.44M | 0.018 | 55.29M | 0.042 | 1.77× | 2.31× |
| 10,000 | 0.142 | 70.42M | 0.132 | 75.83M | 0.152 | 1.07× | 1.15× |
| 100,000 | 1.342 | 74.52M | 1.345 | 74.37M | 0.962 | 0.72× | 0.72× |
| 1,000,000 | 13.795 | 72.49M | 13.727 | 72.85M | 9.251 | 0.67× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.146 | 1.16× |
| 1 | 5 | 0.427 | 0.631 | 1.48× |
| 1 | 10 | 0.759 | 1.250 | 1.65× |
| 10 | 1 | 0.061 | 0.095 | 1.54× |
| 10 | 5 | 0.407 | 0.630 | 1.55× |
| 10 | 10 | 0.714 | 1.289 | 1.81× |
| 100 | 1 | 0.092 | 0.168 | 1.83× |
| 100 | 5 | 0.367 | 0.615 | 1.68× |
| 100 | 10 | 0.717 | 1.272 | 1.77× |
| 1,000 | 1 | 0.100 | 0.126 | 1.26× |
| 1,000 | 5 | 0.360 | 0.579 | 1.61× |
| 1,000 | 10 | 0.747 | 1.309 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
