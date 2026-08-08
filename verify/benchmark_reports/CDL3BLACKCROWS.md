# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.35M | 0.004 | 247.15M | 0.031 | 5.07× | 7.58× |
| 10,000 | 0.056 | 177.73M | 0.051 | 196.75M | 0.084 | 1.49× | 1.64× |
| 100,000 | 0.706 | 141.62M | 0.681 | 146.81M | 0.595 | 0.84× | 0.87× |
| 1,000,000 | 7.343 | 136.19M | 7.165 | 139.56M | 6.045 | 0.82× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.698 ms**; native kernel **0.716 ms**; TA-Lib 0.591 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.362 | 0.261 | 3.83M | 594.762 | 2279.26× | 102.61× |
| 100,000 | 10 | 2.673 | 1.277 | 7.83M | 597.900 | 468.08× | 21.07× |
| 100,000 | 1,000 | 12.394 | 9.516 | 105.09M | 590.998 | 62.11× | 3.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.29M | 115.75M | 1.00× | 2.45M | 2.60M | 1.00× | 134.57M |
| 2 | 223.32M | 236.81M | 2.05× | 2.37M | 2.72M | 1.04× | 134.12M |
| 4 | 338.01M | 444.09M | 3.84× | 2.37M | 2.57M | 0.99× | 137.49M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
