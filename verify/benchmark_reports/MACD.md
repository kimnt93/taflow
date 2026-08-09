# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.28M | 0.005 | 197.10M | 0.051 | 8.24× | 10.06× |
| 10,000 | 0.033 | 305.99M | 0.026 | 391.83M | 0.139 | 4.27× | 5.46× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**; TA-Lib 0.055 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.298 | 0.288 | 3.47M | 55.378 | 192.43× | 143.15× |
| 1,500 | 10 | 1.125 | 0.653 | 15.30M | 56.016 | 85.72× | 66.67× |
| 1,500 | 100 | 4.031 | 2.642 | 37.85M | 58.354 | 22.08× | 16.04× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.20M | 13.03M | 1.00× | 1.37M | 1.40M | 1.00× | 7.12M |
| 2 | 11.75M | 22.44M | 1.72× | 1.49M | 1.38M | 0.98× | 8.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
