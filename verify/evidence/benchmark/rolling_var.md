# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.27M | 0.006 | 156.48M | 0.037 | 5.32× | 5.86× |
| 10,000 | 0.041 | 244.44M | 0.039 | 257.39M | 0.058 | 1.41× | 1.49× |
| 100,000 | 0.378 | 264.67M | 0.368 | 271.63M | 0.265 | 0.70× | 0.72× |
| 1,000,000 | 5.019 | 199.25M | 3.820 | 261.80M | 2.473 | 0.49× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.164 | 1.63× |
| 1 | 5 | 0.314 | 0.570 | 1.82× |
| 1 | 10 | 0.546 | 1.044 | 1.91× |
| 10 | 1 | 0.058 | 0.095 | 1.62× |
| 10 | 5 | 0.280 | 0.610 | 2.18× |
| 10 | 10 | 0.550 | 1.218 | 2.21× |
| 100 | 1 | 0.105 | 0.128 | 1.23× |
| 100 | 5 | 0.294 | 0.515 | 1.75× |
| 100 | 10 | 0.481 | 1.039 | 2.16× |
| 1,000 | 1 | 0.067 | 0.108 | 1.62× |
| 1,000 | 5 | 0.284 | 0.555 | 1.95× |
| 1,000 | 10 | 0.575 | 1.009 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
