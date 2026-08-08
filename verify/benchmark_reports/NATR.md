# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.37M | 0.008 | 129.12M | 0.038 | 4.00× | 4.94× |
| 10,000 | 0.093 | 107.44M | 0.070 | 142.72M | 0.093 | 1.00× | 1.32× |
| 100,000 | 0.699 | 143.10M | 0.653 | 153.23M | 0.624 | 0.89× | 0.96× |
| 1,000,000 | 7.186 | 139.16M | 7.013 | 142.59M | 7.039 | 0.98× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.706 ms**; native kernel **0.648 ms**; TA-Lib 0.599 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.218 | 4.58M | 604.442 | 2770.73× | 146.20× |
| 100,000 | 10 | 1.952 | 0.965 | 10.37M | 613.214 | 635.75× | 34.44× |
| 100,000 | 1,000 | 10.108 | 8.408 | 118.93M | 599.154 | 71.26× | 4.55× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 101.94M | 124.77M | 1.00× | 2.38M | 2.87M | 1.00× | 124.03M |
| 2 | 230.71M | 260.99M | 2.09× | 2.09M | 3.03M | 1.06× | 109.42M |
| 4 | 349.29M | 460.17M | 3.69× | 2.16M | 2.67M | 0.93× | 126.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
