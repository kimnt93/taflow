# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.46M | 0.006 | 169.17M | 0.041 | 5.10× | 6.99× |
| 10,000 | 0.081 | 123.40M | 0.075 | 132.57M | 0.185 | 2.28× | 2.45× |
| 100,000 | 1.132 | 88.36M | 1.051 | 95.18M | 1.551 | 1.37× | 1.48× |
| 1,000,000 | 10.520 | 95.05M | 10.879 | 91.92M | 15.662 | 1.49× | 1.44× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.026 ms**; native kernel **1.022 ms**; TA-Lib 1.704 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.386 | 0.295 | 3.39M | 1536.181 | 5203.07× | 103.61× |
| 100,000 | 10 | 3.282 | 1.740 | 5.75M | 1705.875 | 980.51× | 17.57× |
| 100,000 | 1,000 | 36.715 | 30.576 | 32.71M | 1588.065 | 51.94× | 1.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.77M | 92.55M | 1.00× | 2.50M | 2.31M | 1.00× | 61.63M |
| 2 | 156.67M | 179.67M | 1.94× | 2.38M | 2.30M | 1.00× | 59.77M |
| 4 | 275.45M | 336.07M | 3.63× | 2.24M | 2.35M | 1.02× | 58.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
