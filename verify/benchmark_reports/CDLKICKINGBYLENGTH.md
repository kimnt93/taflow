# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.88M | 0.006 | 163.30M | 0.038 | 4.85× | 6.15× |
| 10,000 | 0.079 | 127.07M | 0.070 | 142.85M | 0.167 | 2.12× | 2.38× |
| 100,000 | 1.026 | 97.50M | 1.007 | 99.26M | 1.420 | 1.38× | 1.41× |
| 1,000,000 | 11.465 | 87.22M | 10.643 | 93.96M | 14.620 | 1.28× | 1.37× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.012 ms**; native kernel **1.014 ms**; TA-Lib 1.483 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.286 | 3.50M | 1488.087 | 5206.20× | 96.60× |
| 100,000 | 10 | 2.617 | 1.334 | 7.50M | 1427.055 | 1070.09× | 21.05× |
| 100,000 | 1,000 | 30.418 | 27.595 | 36.24M | 1464.375 | 53.07× | 1.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 77.78M | 85.16M | 1.00× | 1.90M | 2.04M | 1.00× | 60.30M |
| 2 | 166.73M | 161.69M | 1.90× | 2.43M | 2.61M | 1.28× | 62.22M |
| 4 | 274.29M | 282.46M | 3.32× | 2.17M | 2.54M | 1.24× | 59.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
