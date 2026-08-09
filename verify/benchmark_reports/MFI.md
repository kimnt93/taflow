# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.52M | 0.010 | 99.95M | 0.037 | 3.05× | 3.73× |
| 10,000 | 0.064 | 155.78M | 0.059 | 170.77M | 0.107 | 1.67× | 1.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.013 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.414 | 0.283 | 3.53M | 36.684 | 129.60× | 111.57× |
| 1,500 | 10 | 2.706 | 1.247 | 8.02M | 36.928 | 29.61× | 24.93× |
| 1,500 | 100 | 5.160 | 2.982 | 33.53M | 38.664 | 12.96× | 10.71× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.48M | 17.21M | 1.00× | 990.19K | 1.31M | 1.00× | 7.83M |
| 2 | 12.24M | 20.32M | 1.18× | 1.22M | 1.46M | 1.11× | 8.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
