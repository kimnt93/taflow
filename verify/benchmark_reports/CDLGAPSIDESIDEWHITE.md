# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.49M | 0.004 | 261.54M | 0.044 | 7.85× | 11.63× |
| 10,000 | 0.051 | 195.45M | 0.047 | 214.72M | 0.216 | 4.22× | 4.64× |
| 100,000 | 0.668 | 149.70M | 0.655 | 152.77M | 1.907 | 2.85× | 2.91× |
| 1,000,000 | 7.061 | 141.62M | 6.934 | 144.21M | 18.888 | 2.67× | 2.72× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.669 ms**; native kernel **0.659 ms**; TA-Lib 1.912 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.324 | 0.266 | 3.76M | 1902.709 | 7160.87× | 104.27× |
| 100,000 | 10 | 2.493 | 1.290 | 7.75M | 1867.902 | 1447.44× | 21.81× |
| 100,000 | 1,000 | 27.071 | 24.000 | 41.67M | 1944.457 | 81.02× | 1.86× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 118.28M | 120.43M | 1.00× | 2.24M | 2.48M | 1.00× | 48.64M |
| 2 | 225.84M | 248.72M | 2.07× | 2.46M | 2.59M | 1.04× | 48.50M |
| 4 | 409.93M | 441.09M | 3.66× | 2.27M | 2.29M | 0.92× | 46.93M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
