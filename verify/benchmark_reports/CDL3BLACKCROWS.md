# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.65M | 0.007 | 139.48M | 0.030 | 3.40× | 4.24× |
| 10,000 | 0.054 | 183.54M | 0.054 | 183.92M | 0.083 | 1.52× | 1.52× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.010 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.276 | 3.62M | 32.993 | 119.34× | 105.55× |
| 1,500 | 10 | 2.583 | 1.306 | 7.66M | 33.871 | 25.94× | 20.90× |
| 1,500 | 100 | 5.308 | 3.095 | 32.31M | 33.763 | 10.91× | 9.00× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.79M | 14.74M | 1.00× | 835.03K | 1.33M | 1.00× | 9.37M |
| 2 | 14.92M | 13.89M | 0.94× | 1.23M | 1.16M | 0.87× | 7.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
