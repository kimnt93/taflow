# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 281.31M | 0.001 | 699.66M | 0.033 | 9.17× | 22.81× |
| 10,000 | 0.011 | 932.08M | 0.006 | 1.66G | 0.038 | 3.52× | 6.28× |
| 100,000 | 0.075 | 1.34G | 0.048 | 2.07G | 0.099 | 1.33× | 2.04× |
| 1,000,000 | 1.603 | 623.71M | 1.221 | 818.86M | 1.417 | 0.88× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.075 ms**; native kernel **0.048 ms**; TA-Lib 0.086 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.310 | 0.238 | 4.20M | 82.865 | 347.64× | 114.42× |
| 100,000 | 10 | 1.944 | 1.014 | 9.86M | 81.219 | 80.12× | 26.66× |
| 100,000 | 1,000 | 4.566 | 2.946 | 339.39M | 82.676 | 28.06× | 9.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 410.03M | 610.06M | 1.00× | 2.34M | 3.08M | 1.00× | 502.34M |
| 2 | 781.52M | 1.17G | 1.91× | 2.54M | 3.05M | 0.99× | 508.06M |
| 4 | 823.10M | 1.66G | 2.73× | 2.51M | 2.59M | 0.84× | 538.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
