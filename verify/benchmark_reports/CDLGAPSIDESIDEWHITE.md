# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.20M | 0.004 | 262.37M | 0.057 | 9.96× | 14.91× |
| 10,000 | 0.060 | 167.16M | 0.056 | 177.82M | 0.223 | 3.73× | 3.97× |
| 100,000 | 0.663 | 150.82M | 0.633 | 157.95M | 2.019 | 3.04× | 3.19× |
| 1,000,000 | 7.093 | 140.98M | 7.116 | 140.53M | 20.160 | 2.84× | 2.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.661 ms**; native kernel **0.709 ms**; TA-Lib 2.058 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.285 | 3.51M | 2011.625 | 7057.85× | 95.48× |
| 100,000 | 10 | 2.613 | 1.694 | 5.90M | 1978.126 | 1167.75× | 16.76× |
| 100,000 | 1,000 | 30.303 | 23.629 | 42.32M | 1923.065 | 81.39× | 1.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.99M | 120.33M | 1.00× | 2.10M | 2.24M | 1.00× | 46.21M |
| 2 | 236.88M | 254.04M | 2.11× | 2.42M | 2.67M | 1.19× | 45.28M |
| 4 | 370.15M | 388.87M | 3.23× | 2.35M | 2.66M | 1.19× | 46.31M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
