# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.61M | 0.008 | 130.29M | 0.035 | 3.74× | 4.53× |
| 10,000 | 0.094 | 106.49M | 0.089 | 112.15M | 0.130 | 1.39× | 1.46× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.363 | 0.282 | 3.55M | 39.641 | 140.64× | 97.01× |
| 1,500 | 10 | 2.554 | 1.286 | 7.78M | 38.656 | 30.07× | 21.18× |
| 1,500 | 100 | 5.508 | 3.299 | 30.32M | 40.891 | 12.40× | 8.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.37M | 14.31M | 1.00× | 1.17M | 894.65K | 1.00× | 9.23M |
| 2 | 18.81M | 19.70M | 1.38× | 1.18M | 1.32M | 1.47× | 9.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
