# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.08M | 0.004 | 226.35M | 0.033 | 5.38× | 7.43× |
| 10,000 | 0.060 | 166.02M | 0.055 | 180.95M | 0.108 | 1.80× | 1.96× |
| 100,000 | 0.654 | 153.02M | 0.643 | 155.49M | 0.809 | 1.24× | 1.26× |
| 1,000,000 | 7.030 | 142.24M | 6.797 | 147.12M | 8.261 | 1.18× | 1.22× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.655 ms**; native kernel **0.640 ms**; TA-Lib 0.819 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.252 | 3.96M | 818.252 | 3241.96× | 109.12× |
| 100,000 | 10 | 2.576 | 1.282 | 7.80M | 822.437 | 641.53× | 21.19× |
| 100,000 | 1,000 | 27.141 | 24.955 | 40.07M | 833.427 | 33.40× | 1.29× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 105.53M | 113.46M | 1.00× | 2.11M | 2.80M | 1.00× | 96.91M |
| 2 | 229.98M | 241.09M | 2.12× | 2.39M | 2.89M | 1.03× | 96.49M |
| 4 | 386.10M | 439.81M | 3.88× | 2.26M | 2.49M | 0.89× | 100.12M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
