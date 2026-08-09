# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.92M | 0.045 | 22.26M | 0.072 | 1.51× | 1.61× |
| 10,000 | 0.432 | 23.14M | 0.437 | 22.91M | 0.449 | 1.04× | 1.03× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.075 ms**; TA-Lib 0.091 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.279 | 0.217 | 4.61M | 94.534 | 436.04× | 128.63× |
| 1,500 | 10 | 1.521 | 0.992 | 10.08M | 90.571 | 91.28× | 28.05× |
| 1,500 | 100 | 7.167 | 5.960 | 16.78M | 95.882 | 16.09× | 5.53× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.57M | 8.39M | 1.00× | 1.26M | 1.31M | 1.00× | 6.79M |
| 2 | 11.61M | 14.61M | 1.74× | 1.39M | 1.57M | 1.20× | 6.81M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
