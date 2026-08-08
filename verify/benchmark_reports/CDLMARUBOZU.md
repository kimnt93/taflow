# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.23M | 0.005 | 210.85M | 0.039 | 5.45× | 8.14× |
| 10,000 | 0.077 | 129.55M | 0.073 | 137.59M | 0.133 | 1.73× | 1.83× |
| 100,000 | 0.943 | 106.02M | 0.929 | 107.60M | 1.103 | 1.17× | 1.19× |
| 1,000,000 | 9.987 | 100.13M | 9.795 | 102.10M | 11.126 | 1.11× | 1.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.938 ms**; native kernel **0.919 ms**; TA-Lib 1.106 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.376 | 0.289 | 3.46M | 1100.488 | 3808.89× | 95.84× |
| 100,000 | 10 | 2.709 | 1.382 | 7.23M | 1109.449 | 802.66× | 20.77× |
| 100,000 | 1,000 | 34.845 | 40.575 | 24.65M | 1120.484 | 27.62× | 0.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.45M | 95.32M | 1.00× | 2.17M | 2.25M | 1.00× | 76.56M |
| 2 | 169.02M | 180.04M | 1.89× | 2.30M | 2.59M | 1.15× | 78.44M |
| 4 | 313.78M | 360.03M | 3.78× | 2.31M | 2.45M | 1.09× | 78.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
