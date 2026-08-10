# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.73M | 0.007 | 139.62M | 0.200 | 21.70× | 27.86× |
| 10,000 | 0.030 | 329.49M | 0.026 | 387.28M | 1.088 | 35.85× | 42.14× |
| 100,000 | 0.253 | 394.96M | 0.298 | 335.34M | 9.860 | 38.94× | 33.06× |
| 1,000,000 | 2.819 | 354.71M | 2.442 | 409.45M | 99.877 | 35.43× | 40.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.221 | 2.13× |
| 1 | 5 | 0.316 | 0.822 | 2.60× |
| 1 | 10 | 0.562 | 1.986 | 3.53× |
| 10 | 1 | 0.057 | 0.168 | 2.97× |
| 10 | 5 | 0.244 | 0.849 | 3.48× |
| 10 | 10 | 0.581 | 1.993 | 3.43× |
| 100 | 1 | 0.062 | 0.174 | 2.82× |
| 100 | 5 | 0.244 | 0.878 | 3.60× |
| 100 | 10 | 0.571 | 1.835 | 3.22× |
| 1,000 | 1 | 0.061 | 0.260 | 4.24× |
| 1,000 | 5 | 0.272 | 1.701 | 6.26× |
| 1,000 | 10 | 0.571 | 2.735 | 4.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
