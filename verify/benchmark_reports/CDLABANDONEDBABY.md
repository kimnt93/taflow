# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.60M | 0.006 | 181.48M | 0.037 | 5.21× | 6.77× |
| 10,000 | 0.099 | 101.47M | 0.096 | 104.20M | 0.138 | 1.40× | 1.43× |
| 100,000 | 1.066 | 93.80M | 1.035 | 96.61M | 1.082 | 1.01× | 1.04× |
| 1,000,000 | 10.871 | 91.99M | 10.777 | 92.79M | 11.079 | 1.02× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.082 ms**; native kernel **1.055 ms**; TA-Lib 1.102 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.358 | 0.302 | 3.31M | 1085.483 | 3594.30× | 103.08× |
| 100,000 | 10 | 2.710 | 1.504 | 6.65M | 1092.359 | 726.22× | 21.01× |
| 100,000 | 1,000 | 31.692 | 30.057 | 33.27M | 1106.866 | 36.83× | 1.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78.94M | 89.19M | 1.00× | 2.05M | 2.07M | 1.00× | 77.92M |
| 2 | 157.06M | 159.42M | 1.79× | 2.31M | 2.65M | 1.28× | 77.53M |
| 4 | 304.44M | 271.20M | 3.04× | 2.28M | 2.53M | 1.23× | 81.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
