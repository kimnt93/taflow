# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.896 | 1.12M | 0.913 | 1.10M | 1.593 | 1.78× | 1.75× |
| 10,000 | 10.206 | 979.85K | 9.563 | 1.05M | 13.195 | 1.29× | 1.38× |
| 100,000 | 96.927 | 1.03M | 96.757 | 1.03M | 137.662 | 1.42× | 1.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.298 | 2.90× |
| 1 | 5 | 0.322 | 1.428 | 4.44× |
| 1 | 10 | 0.470 | 2.712 | 5.77× |
| 10 | 1 | 0.056 | 0.281 | 5.06× |
| 10 | 5 | 0.250 | 1.467 | 5.86× |
| 10 | 10 | 0.521 | 2.933 | 5.63× |
| 100 | 1 | 0.114 | 0.363 | 3.17× |
| 100 | 5 | 0.267 | 2.064 | 7.73× |
| 100 | 10 | 0.568 | 3.759 | 6.61× |
| 1,000 | 1 | 1.007 | 1.713 | 1.70× |
| 1,000 | 5 | 1.152 | 8.599 | 7.47× |
| 1,000 | 10 | 1.973 | 17.224 | 8.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
