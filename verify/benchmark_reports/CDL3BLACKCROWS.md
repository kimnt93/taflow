# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.31M | 0.012 | 83.59M | 0.032 | 2.36× | 2.65× |
| 10,000 | 0.147 | 68.06M | 0.142 | 70.31M | 0.084 | 0.57× | 0.59× |
| 100,000 | 1.477 | 67.72M | 1.461 | 68.46M | 0.613 | 0.41× | 0.42× |
| 1,000,000 | 15.104 | 66.21M | 14.655 | 68.24M | 6.319 | 0.42× | 0.43× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.450 ms**; native kernel **1.475 ms**; TA-Lib 0.619 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.348 | 0.284 | 3.52M | 628.317 | 2211.32× | 94.43× |
| 100,000 | 10 | 2.629 | 1.464 | 6.83M | 605.007 | 413.21× | 19.44× |
| 100,000 | 1,000 | 38.373 | 36.430 | 27.45M | 629.295 | 17.27× | 0.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 57.67M | 62.87M | 1.00× | 2.25M | 1.95M | 1.00× | 125.86M |
| 2 | 117.49M | 114.33M | 1.82× | 2.37M | 2.17M | 1.11× | 126.79M |
| 4 | 115.28M | 98.87M | 1.57× | 2.04M | 2.44M | 1.25× | 120.83M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
