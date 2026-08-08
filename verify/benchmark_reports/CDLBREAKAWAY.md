# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.06M | 0.004 | 223.61M | 0.030 | 4.59× | 6.80× |
| 10,000 | 0.105 | 94.92M | 0.103 | 97.34M | 0.091 | 0.86× | 0.88× |
| 100,000 | 1.303 | 76.77M | 1.201 | 83.25M | 0.680 | 0.52× | 0.57× |
| 1,000,000 | 12.318 | 81.18M | 12.343 | 81.02M | 7.085 | 0.58× | 0.57× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.201 ms**; native kernel **1.176 ms**; TA-Lib 0.660 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.353 | 0.295 | 3.39M | 653.773 | 2217.28× | 95.11× |
| 100,000 | 10 | 2.603 | 1.483 | 6.74M | 673.439 | 454.16× | 19.53× |
| 100,000 | 1,000 | 34.429 | 31.490 | 31.76M | 675.419 | 21.45× | 1.05× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 69.40M | 75.12M | 1.00× | 2.16M | 2.52M | 1.00× | 121.02M |
| 2 | 142.30M | 134.38M | 1.79× | 2.18M | 2.44M | 0.97× | 113.85M |
| 4 | 254.58M | 269.42M | 3.59× | 2.20M | 2.46M | 0.98× | 123.86M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
