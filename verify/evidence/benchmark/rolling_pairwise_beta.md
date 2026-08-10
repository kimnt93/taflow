# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.55M | 0.034 | 29.74M | 0.241 | 7.12× | 7.17× |
| 10,000 | 0.305 | 32.80M | 0.286 | 34.99M | 1.034 | 3.39× | 3.62× |
| 100,000 | 2.925 | 34.19M | 2.856 | 35.01M | 9.255 | 3.16× | 3.24× |
| 1,000,000 | 31.995 | 31.25M | 29.137 | 34.32M | 92.326 | 2.89× | 3.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.324 | 2.84× |
| 1 | 5 | 0.272 | 1.434 | 5.27× |
| 1 | 10 | 0.557 | 2.661 | 4.78× |
| 10 | 1 | 0.061 | 0.223 | 3.67× |
| 10 | 5 | 0.252 | 1.344 | 5.34× |
| 10 | 10 | 0.537 | 2.683 | 5.00× |
| 100 | 1 | 0.067 | 0.236 | 3.55× |
| 100 | 5 | 0.277 | 1.395 | 5.03× |
| 100 | 10 | 0.622 | 2.643 | 4.25× |
| 1,000 | 1 | 0.097 | 0.396 | 4.09× |
| 1,000 | 5 | 0.320 | 1.797 | 5.62× |
| 1,000 | 10 | 0.603 | 4.254 | 7.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
