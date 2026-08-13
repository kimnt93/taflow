# KalmanHedgeRatio benchmark (`KalmanHedgeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.82M | 0.037 | 27.22M | 0.548 | 13.06× | 14.92× |
| 10,000 | 0.286 | 34.91M | 0.274 | 36.45M | 3.697 | 12.91× | 13.47× |
| 100,000 | 2.739 | 36.51M | 2.656 | 37.65M | 40.872 | 14.92× | 15.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.329 | 2.70× |
| 1 | 5 | 0.470 | 13.239 | 28.15× |
| 1 | 10 | 0.751 | 2.787 | 3.71× |
| 10 | 1 | 0.078 | 0.250 | 3.20× |
| 10 | 5 | 0.300 | 1.480 | 4.94× |
| 10 | 10 | 0.597 | 2.703 | 4.52× |
| 100 | 1 | 0.076 | 0.290 | 3.82× |
| 100 | 5 | 0.309 | 1.676 | 5.42× |
| 100 | 10 | 0.629 | 3.236 | 5.14× |
| 1,000 | 1 | 0.100 | 0.819 | 8.23× |
| 1,000 | 5 | 0.297 | 3.704 | 12.48× |
| 1,000 | 10 | 0.631 | 7.368 | 11.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
