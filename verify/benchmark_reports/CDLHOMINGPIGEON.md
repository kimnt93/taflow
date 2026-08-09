# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.05M | 0.008 | 127.06M | 0.033 | 3.61× | 4.24× |
| 10,000 | 0.060 | 166.01M | 0.058 | 172.24M | 0.110 | 1.83× | 1.90× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.008 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.374 | 0.334 | 2.99M | 34.039 | 101.91× | 89.41× |
| 1,500 | 10 | 2.831 | 1.850 | 5.41M | 35.694 | 19.30× | 16.38× |
| 1,500 | 100 | 5.562 | 3.232 | 30.94M | 34.425 | 10.65× | 11.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.53M | 15.34M | 1.00× | 884.22K | 1.03M | 1.00× | 8.12M |
| 2 | 16.58M | 18.75M | 1.22× | 1.25M | 1.33M | 1.30× | 9.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
