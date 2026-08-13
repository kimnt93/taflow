# ButterflyPattern benchmark (`Butterfly` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 17.05M | 0.053 | 18.87M | 0.212 | 3.62× | 4.01× |
| 10,000 | 0.386 | 25.89M | 0.386 | 25.88M | 1.268 | 3.28× | 3.28× |
| 100,000 | 3.722 | 26.87M | 3.625 | 27.58M | 12.008 | 3.23× | 3.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.202 | 1.54× |
| 1 | 5 | 0.484 | 0.865 | 1.79× |
| 1 | 10 | 0.637 | 1.667 | 2.61× |
| 10 | 1 | 0.073 | 0.164 | 2.24× |
| 10 | 5 | 0.320 | 1.116 | 3.48× |
| 10 | 10 | 0.633 | 1.653 | 2.61× |
| 100 | 1 | 0.078 | 0.180 | 2.30× |
| 100 | 5 | 0.315 | 7.215 | 22.89× |
| 100 | 10 | 0.736 | 1.835 | 2.49× |
| 1,000 | 1 | 0.116 | 0.291 | 2.51× |
| 1,000 | 5 | 0.343 | 1.434 | 4.18× |
| 1,000 | 10 | 0.686 | 2.965 | 4.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
