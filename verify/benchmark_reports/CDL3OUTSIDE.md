# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 208.98M | 0.003 | 333.31M | 0.030 | 6.20× | 9.88× |
| 10,000 | 0.063 | 159.72M | 0.060 | 166.60M | 0.081 | 1.29× | 1.35× |
| 100,000 | 0.720 | 138.93M | 0.708 | 141.30M | 0.554 | 0.77× | 0.78× |
| 1,000,000 | 7.650 | 130.72M | 7.340 | 136.24M | 5.392 | 0.70× | 0.73× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.727 ms**; native kernel **0.700 ms**; TA-Lib 0.553 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.314 | 0.256 | 3.91M | 581.685 | 2273.90× | 106.80× |
| 100,000 | 10 | 2.460 | 1.234 | 8.11M | 568.419 | 460.75× | 21.78× |
| 100,000 | 1,000 | 21.451 | 19.281 | 51.86M | 565.339 | 29.32× | 1.48× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 107.22M | 109.20M | 1.00× | 2.10M | 2.49M | 1.00× | 134.64M |
| 2 | 207.70M | 211.41M | 1.94× | 2.45M | 2.57M | 1.03× | 137.68M |
| 4 | 352.04M | 407.37M | 3.73× | 2.36M | 2.56M | 1.03× | 133.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
