# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.78M | 0.004 | 228.00M | 0.035 | 5.53× | 7.89× |
| 10,000 | 0.112 | 89.30M | 0.099 | 100.56M | 0.180 | 1.61× | 1.81× |
| 100,000 | 1.151 | 86.85M | 1.139 | 87.80M | 1.360 | 1.18× | 1.19× |
| 1,000,000 | 11.842 | 84.44M | 11.851 | 84.38M | 13.603 | 1.15× | 1.15× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.150 ms**; native kernel **1.162 ms**; TA-Lib 1.387 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.307 | 3.26M | 1365.744 | 4447.43× | 89.33× |
| 100,000 | 10 | 2.778 | 1.476 | 6.77M | 1365.622 | 925.15× | 18.35× |
| 100,000 | 1,000 | 30.131 | 27.730 | 36.06M | 1373.256 | 49.52× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.49M | 73.23M | 1.00× | 2.10M | 2.63M | 1.00× | 58.87M |
| 2 | 151.02M | 163.87M | 2.24× | 2.44M | 2.56M | 0.98× | 68.32M |
| 4 | 269.07M | 284.87M | 3.89× | 2.30M | 2.30M | 0.88× | 63.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
