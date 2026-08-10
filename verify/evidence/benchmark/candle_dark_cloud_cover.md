# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.52M | 0.014 | 72.62M | 0.036 | 2.13× | 2.64× |
| 10,000 | 0.131 | 76.40M | 0.127 | 78.96M | 0.142 | 1.08× | 1.12× |
| 100,000 | 1.316 | 75.97M | 1.334 | 74.97M | 0.932 | 0.71× | 0.70× |
| 1,000,000 | 13.480 | 74.18M | 12.713 | 78.66M | 8.201 | 0.61× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.124 | 1.24× |
| 1 | 5 | 0.336 | 0.586 | 1.75× |
| 1 | 10 | 0.623 | 1.073 | 1.72× |
| 10 | 1 | 0.063 | 0.097 | 1.54× |
| 10 | 5 | 0.315 | 0.566 | 1.79× |
| 10 | 10 | 0.641 | 1.101 | 1.72× |
| 100 | 1 | 0.062 | 0.095 | 1.54× |
| 100 | 5 | 0.353 | 0.532 | 1.51× |
| 100 | 10 | 0.608 | 1.081 | 1.78× |
| 1,000 | 1 | 0.069 | 0.111 | 1.61× |
| 1,000 | 5 | 0.309 | 0.573 | 1.86× |
| 1,000 | 10 | 0.713 | 1.223 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
