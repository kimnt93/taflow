# SmoothedTrendChannel benchmark (`smoothed trend channel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.70M | 0.015 | 65.24M | 0.590 | 31.10× | 38.49× |
| 10,000 | 0.146 | 68.35M | 0.116 | 86.04M | 5.565 | 38.04× | 47.88× |
| 100,000 | 1.441 | 69.37M | 1.179 | 84.83M | 52.154 | 36.18× | 44.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.157 | 1.38× |
| 1 | 5 | 0.378 | 0.651 | 1.72× |
| 1 | 10 | 0.537 | 1.197 | 2.23× |
| 10 | 1 | 0.053 | 0.189 | 3.57× |
| 10 | 5 | 0.295 | 0.906 | 3.07× |
| 10 | 10 | 0.555 | 1.796 | 3.23× |
| 100 | 1 | 0.057 | 0.210 | 3.69× |
| 100 | 5 | 0.263 | 1.113 | 4.23× |
| 100 | 10 | 0.529 | 2.374 | 4.49× |
| 1,000 | 1 | 0.066 | 0.696 | 10.61× |
| 1,000 | 5 | 0.259 | 3.525 | 13.59× |
| 1,000 | 10 | 0.538 | 6.861 | 12.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
