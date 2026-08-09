# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.43M | 0.007 | 137.35M | 0.034 | 3.72× | 4.63× |
| 10,000 | 0.076 | 131.82M | 0.072 | 138.36M | 0.115 | 1.51× | 1.59× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.346 | 0.267 | 3.75M | 37.098 | 139.00× | 107.19× |
| 1,500 | 10 | 2.563 | 1.293 | 7.74M | 37.709 | 29.17× | 21.87× |
| 1,500 | 100 | 5.741 | 3.275 | 30.54M | 38.055 | 11.62× | 8.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.66M | 13.81M | 1.00× | 1.17M | 1.23M | 1.00× | 9.57M |
| 2 | 11.71M | 19.92M | 1.44× | 1.34M | 1.38M | 1.13× | 9.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
