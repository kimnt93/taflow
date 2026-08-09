# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.11M | 0.005 | 216.25M | 0.041 | 6.20× | 8.76× |
| 10,000 | 0.096 | 104.01M | 0.091 | 109.62M | 0.161 | 1.68× | 1.77× |
| 100,000 | 1.122 | 89.16M | 1.106 | 90.38M | 1.333 | 1.19× | 1.20× |
| 1,000,000 | 11.870 | 84.25M | 11.751 | 85.10M | 13.070 | 1.10× | 1.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.117 ms**; native kernel **1.109 ms**; TA-Lib 1.296 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.262 | 3.82M | 1327.116 | 5063.74× | 104.46× |
| 100,000 | 10 | 2.528 | 1.313 | 7.62M | 1306.259 | 995.16× | 20.72× |
| 100,000 | 1,000 | 30.493 | 30.320 | 32.98M | 1343.091 | 44.30× | 1.29× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 72.23M | 71.93M | 1.00× | 2.13M | 2.24M | 1.00× | 61.36M |
| 2 | 140.57M | 155.58M | 2.16× | 2.52M | 2.63M | 1.18× | 65.36M |
| 4 | 249.96M | 277.73M | 3.86× | 2.30M | 2.32M | 1.04× | 65.37M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
