# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.83M | 0.005 | 185.26M | 0.034 | 4.82× | 6.34× |
| 10,000 | 0.121 | 82.43M | 0.116 | 85.85M | 0.192 | 1.59× | 1.65× |
| 100,000 | 1.325 | 75.48M | 1.260 | 79.39M | 1.659 | 1.25× | 1.32× |
| 1,000,000 | 13.241 | 75.52M | 13.134 | 76.14M | 16.847 | 1.27× | 1.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.317 ms**; native kernel **1.266 ms**; TA-Lib 1.657 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.330 | 0.259 | 3.86M | 1667.395 | 6430.72× | 107.67× |
| 100,000 | 10 | 2.582 | 1.370 | 7.30M | 1665.738 | 1216.07× | 19.54× |
| 100,000 | 1,000 | 27.590 | 23.330 | 42.86M | 1650.458 | 70.75× | 1.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 68.46M | 66.38M | 1.00× | 2.13M | 2.12M | 1.00× | 52.96M |
| 2 | 128.42M | 130.73M | 1.97× | 2.17M | 2.38M | 1.12× | 52.84M |
| 4 | 231.09M | 233.15M | 3.51× | 2.31M | 2.51M | 1.18× | 51.56M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
