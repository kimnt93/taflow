# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.74M | 0.004 | 232.91M | 0.034 | 5.45× | 8.00× |
| 10,000 | 0.060 | 165.60M | 0.055 | 182.93M | 0.130 | 2.16× | 2.38× |
| 100,000 | 0.841 | 118.85M | 0.823 | 121.48M | 0.978 | 1.16× | 1.19× |
| 1,000,000 | 8.962 | 111.59M | 9.070 | 110.25M | 9.367 | 1.05× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.859 ms**; native kernel **0.845 ms**; TA-Lib 0.953 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.463 | 0.295 | 3.38M | 972.049 | 3289.82× | 102.06× |
| 100,000 | 10 | 2.981 | 1.396 | 7.17M | 987.813 | 707.79× | 20.40× |
| 100,000 | 1,000 | 29.473 | 27.619 | 36.21M | 974.152 | 35.27× | 1.14× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 94.51M | 99.84M | 1.00× | 1.81M | 2.32M | 1.00× | 84.42M |
| 2 | 187.75M | 205.76M | 2.06× | 2.18M | 2.53M | 1.09× | 88.94M |
| 4 | 325.06M | 366.43M | 3.67× | 2.19M | 2.42M | 1.04× | 91.57M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
