# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.22M | 0.003 | 298.50M | 0.030 | 5.86× | 8.96× |
| 10,000 | 0.045 | 222.34M | 0.040 | 251.36M | 0.088 | 1.95× | 2.21× |
| 100,000 | 0.479 | 208.98M | 0.472 | 212.01M | 0.636 | 1.33× | 1.35× |
| 1,000,000 | 5.228 | 191.27M | 5.210 | 191.93M | 6.465 | 1.24× | 1.24× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.476 ms**; native kernel **0.470 ms**; TA-Lib 0.637 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.341 | 0.273 | 3.67M | 630.792 | 2314.73× | 100.54× |
| 100,000 | 10 | 2.577 | 1.321 | 7.57M | 631.631 | 478.25× | 20.47× |
| 100,000 | 1,000 | 23.170 | 19.506 | 51.27M | 656.755 | 33.67× | 1.53× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 155.42M | 152.32M | 1.00× | 2.29M | 2.80M | 1.00× | 127.43M |
| 2 | 291.97M | 317.93M | 2.09× | 2.33M | 2.66M | 0.95× | 131.68M |
| 4 | 500.46M | 588.33M | 3.86× | 2.33M | 2.69M | 0.96× | 128.49M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
