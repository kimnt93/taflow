# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.56M | 0.004 | 243.87M | 0.032 | 5.29× | 7.75× |
| 10,000 | 0.059 | 170.06M | 0.057 | 176.41M | 0.079 | 1.35× | 1.40× |
| 100,000 | 0.882 | 113.32M | 0.858 | 116.60M | 0.573 | 0.65× | 0.67× |
| 1,000,000 | 9.343 | 107.03M | 9.022 | 110.84M | 5.680 | 0.61× | 0.63× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.881 ms**; native kernel **0.857 ms**; TA-Lib 0.570 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.334 | 0.275 | 3.63M | 589.370 | 2142.28× | 102.47× |
| 100,000 | 10 | 2.514 | 1.322 | 7.56M | 576.843 | 436.24× | 20.56× |
| 100,000 | 1,000 | 28.915 | 26.415 | 37.86M | 576.363 | 21.82× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.01M | 97.58M | 1.00× | 2.12M | 2.71M | 1.00× | 143.99M |
| 2 | 194.17M | 179.26M | 1.84× | 2.23M | 2.50M | 0.92× | 139.51M |
| 4 | 345.40M | 369.09M | 3.78× | 2.39M | 2.60M | 0.96× | 141.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
