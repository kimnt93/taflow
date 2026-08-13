# RollingAutocorr benchmark (`Autocorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.468 | 2.14M | 0.457 | 2.19M | 0.246 | 0.53× | 0.54× |
| 10,000 | 4.629 | 2.16M | 4.565 | 2.19M | 1.086 | 0.23× | 0.24× |
| 100,000 | 45.974 | 2.18M | 44.700 | 2.24M | 9.213 | 0.20× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.259 | 1.54× |
| 1 | 5 | 0.353 | 1.464 | 4.14× |
| 1 | 10 | 0.592 | 2.601 | 4.40× |
| 10 | 1 | 0.080 | 0.235 | 2.92× |
| 10 | 5 | 0.313 | 1.458 | 4.65× |
| 10 | 10 | 0.598 | 2.473 | 4.14× |
| 100 | 1 | 0.113 | 0.249 | 2.22× |
| 100 | 5 | 0.313 | 1.466 | 4.68× |
| 100 | 10 | 0.636 | 2.748 | 4.32× |
| 1,000 | 1 | 0.537 | 0.335 | 0.62× |
| 1,000 | 5 | 0.863 | 1.928 | 2.23× |
| 1,000 | 10 | 1.302 | 3.525 | 2.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
