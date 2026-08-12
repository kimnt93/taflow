# ProjectionBands benchmark (`rolling projection mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.53M | 0.017 | 59.62M | 0.086 | 4.78× | 5.13× |
| 10,000 | 0.148 | 67.69M | 0.148 | 67.70M | 0.275 | 1.86× | 1.86× |
| 100,000 | 1.446 | 69.18M | 1.515 | 66.01M | 2.240 | 1.55× | 1.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.133 | 1.38× |
| 1 | 5 | 0.307 | 0.507 | 1.65× |
| 1 | 10 | 0.466 | 1.046 | 2.24× |
| 10 | 1 | 0.050 | 0.103 | 2.08× |
| 10 | 5 | 0.244 | 0.521 | 2.14× |
| 10 | 10 | 0.474 | 1.061 | 2.24× |
| 100 | 1 | 0.053 | 0.141 | 2.68× |
| 100 | 5 | 0.235 | 0.692 | 2.94× |
| 100 | 10 | 0.489 | 1.430 | 2.92× |
| 1,000 | 1 | 0.066 | 0.161 | 2.46× |
| 1,000 | 5 | 0.244 | 0.742 | 3.04× |
| 1,000 | 10 | 0.540 | 1.630 | 3.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
