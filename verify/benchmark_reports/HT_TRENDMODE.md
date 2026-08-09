# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.164 | 6.08M | 0.167 | 6.00M | 0.471 | 2.86× | 2.83× |
| 10,000 | 1.661 | 6.02M | 1.659 | 6.03M | 4.726 | 2.84× | 2.85× |
| 100,000 | 17.766 | 5.63M | 16.916 | 5.91M | 45.525 | 2.56× | 2.69× |
| 1,000,000 | 168.383 | 5.94M | 169.669 | 5.89M | 488.445 | 2.90× | 2.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.113 | 1.18× |
| 1 | 5 | 0.251 | 0.455 | 1.81× |
| 1 | 10 | 0.459 | 0.901 | 1.96× |
| 10 | 1 | 0.048 | 0.089 | 1.86× |
| 10 | 5 | 0.221 | 0.423 | 1.91× |
| 10 | 10 | 0.451 | 0.906 | 2.01× |
| 100 | 1 | 0.063 | 0.115 | 1.80× |
| 100 | 5 | 0.217 | 0.568 | 2.62× |
| 100 | 10 | 0.504 | 1.211 | 2.40× |
| 1,000 | 1 | 0.229 | 0.568 | 2.48× |
| 1,000 | 5 | 0.381 | 2.877 | 7.56× |
| 1,000 | 10 | 0.646 | 5.667 | 8.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
