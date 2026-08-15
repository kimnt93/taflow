# KeltnerChannels benchmark (`Keltner` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.25M | 0.012 | 85.22M | 0.637 | 40.31× | 54.32× |
| 10,000 | 0.104 | 96.62M | 0.091 | 109.80M | 4.140 | 40.00× | 45.46× |
| 100,000 | 0.990 | 101.04M | 0.937 | 106.70M | 46.197 | 46.68× | 49.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.334 | 4.51× |
| 1 | 5 | 0.239 | 1.619 | 6.78× |
| 1 | 10 | 0.416 | 3.208 | 7.71× |
| 10 | 1 | 0.048 | 0.284 | 5.89× |
| 10 | 5 | 0.188 | 1.546 | 8.22× |
| 10 | 10 | 0.421 | 3.240 | 7.69× |
| 100 | 1 | 0.049 | 0.323 | 6.54× |
| 100 | 5 | 0.220 | 1.801 | 8.20× |
| 100 | 10 | 0.445 | 3.646 | 8.20× |
| 1,000 | 1 | 0.060 | 0.812 | 13.64× |
| 1,000 | 5 | 0.216 | 3.930 | 18.19× |
| 1,000 | 10 | 0.432 | 7.762 | 17.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
