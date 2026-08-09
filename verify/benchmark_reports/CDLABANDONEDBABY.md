# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.04M | 0.008 | 121.05M | 0.037 | 3.46× | 4.50× |
| 10,000 | 0.100 | 100.09M | 0.098 | 102.31M | 0.132 | 1.32× | 1.35× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.351 | 0.350 | 2.86M | 42.356 | 121.07× | 94.87× |
| 1,500 | 10 | 2.576 | 1.285 | 7.78M | 41.743 | 32.49× | 26.28× |
| 1,500 | 100 | 5.853 | 3.542 | 28.23M | 51.630 | 14.58× | 9.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.62M | 14.83M | 1.00× | 639.29K | 1.20M | 1.00× | 8.50M |
| 2 | 11.81M | 16.19M | 1.09× | 1.20M | 1.36M | 1.14× | 8.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
