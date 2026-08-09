# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.66M | 0.004 | 223.78M | 0.034 | 5.37× | 7.67× |
| 10,000 | 0.099 | 100.62M | 0.096 | 104.13M | 0.155 | 1.56× | 1.61× |
| 100,000 | 1.178 | 84.92M | 1.165 | 85.87M | 1.295 | 1.10× | 1.11× |
| 1,000,000 | 12.168 | 82.18M | 11.657 | 85.79M | 12.742 | 1.05× | 1.09× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.164 ms**; native kernel **1.150 ms**; TA-Lib 1.281 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.349 | 0.270 | 3.71M | 1296.751 | 4808.77× | 101.48× |
| 100,000 | 10 | 2.557 | 1.408 | 7.10M | 1278.659 | 907.88× | 19.96× |
| 100,000 | 1,000 | 29.124 | 29.349 | 34.07M | 1297.410 | 44.21× | 1.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 67.59M | 76.80M | 1.00× | 2.09M | 2.34M | 1.00× | 68.53M |
| 2 | 141.78M | 146.99M | 1.91× | 2.24M | 2.60M | 1.11× | 66.82M |
| 4 | 250.02M | 279.33M | 3.64× | 2.15M | 2.38M | 1.02× | 67.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
