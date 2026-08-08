# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.75M | 0.005 | 189.12M | 0.033 | 4.58× | 6.29× |
| 10,000 | 0.069 | 144.37M | 0.069 | 145.34M | 0.115 | 1.66× | 1.67× |
| 100,000 | 0.796 | 125.55M | 0.781 | 128.04M | 0.903 | 1.13× | 1.16× |
| 1,000,000 | 8.401 | 119.03M | 8.285 | 120.70M | 9.016 | 1.07× | 1.09× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.771 ms**; native kernel **0.759 ms**; TA-Lib 0.897 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.361 | 0.287 | 3.48M | 879.986 | 3066.09× | 101.42× |
| 100,000 | 10 | 2.959 | 1.772 | 5.64M | 942.019 | 531.63× | 15.20× |
| 100,000 | 1,000 | 27.882 | 26.309 | 38.01M | 890.768 | 33.86× | 1.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.46M | 117.36M | 1.00× | 1.85M | 2.43M | 1.00× | 90.30M |
| 2 | 202.60M | 227.03M | 1.93× | 2.17M | 2.44M | 1.01× | 93.98M |
| 4 | 352.75M | 418.06M | 3.56× | 2.45M | 2.48M | 1.02× | 99.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
