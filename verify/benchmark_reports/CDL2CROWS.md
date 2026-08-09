# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.49M | 0.007 | 151.17M | 0.031 | 3.51× | 4.72× |
| 10,000 | 0.064 | 155.18M | 0.060 | 165.77M | 0.109 | 1.69× | 1.81× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.353 | 0.276 | 3.62M | 30.478 | 110.32× | 96.47× |
| 1,500 | 10 | 2.533 | 1.264 | 7.91M | 31.032 | 24.56× | 22.45× |
| 1,500 | 100 | 5.442 | 3.119 | 32.06M | 32.298 | 10.36× | 10.79× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.82M | 15.82M | 1.00× | 929.79K | 1.26M | 1.00× | 7.59M |
| 2 | 14.56M | 19.26M | 1.22× | 1.16M | 1.46M | 1.16× | 10.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
