# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.15M | 0.006 | 160.11M | 0.045 | 6.34× | 7.19× |
| 10,000 | 0.086 | 115.63M | 0.081 | 123.31M | 0.144 | 1.66× | 1.77× |
| 100,000 | 1.042 | 95.99M | 1.032 | 96.87M | 1.172 | 1.13× | 1.14× |
| 1,000,000 | 10.911 | 91.65M | 10.659 | 93.82M | 11.325 | 1.04× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.118 ms**; native kernel **1.045 ms**; TA-Lib 1.149 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.369 | 0.296 | 3.38M | 1172.525 | 3957.46× | 97.66× |
| 100,000 | 10 | 2.831 | 1.778 | 5.62M | 1147.871 | 645.67× | 15.94× |
| 100,000 | 1,000 | 30.389 | 28.312 | 35.32M | 1161.643 | 41.03× | 1.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.51M | 82.88M | 1.00× | 2.03M | 2.12M | 1.00× | 74.54M |
| 2 | 162.11M | 163.22M | 1.97× | 2.28M | 2.30M | 1.08× | 77.99M |
| 4 | 246.87M | 318.47M | 3.84× | 2.25M | 2.45M | 1.15× | 77.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
