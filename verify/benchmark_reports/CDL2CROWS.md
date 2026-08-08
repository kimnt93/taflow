# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.26M | 0.004 | 265.33M | 0.031 | 4.67× | 8.14× |
| 10,000 | 0.061 | 162.89M | 0.060 | 165.64M | 0.105 | 1.72× | 1.74× |
| 100,000 | 0.916 | 109.22M | 0.872 | 114.62M | 0.867 | 0.95× | 0.99× |
| 1,000,000 | 9.197 | 108.73M | 9.015 | 110.92M | 8.561 | 0.93× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.899 ms**; native kernel **0.877 ms**; TA-Lib 0.878 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.365 | 0.312 | 3.21M | 856.889 | 2748.20× | 86.60× |
| 100,000 | 10 | 2.744 | 1.695 | 5.90M | 866.357 | 511.10× | 16.00× |
| 100,000 | 1,000 | 37.260 | 30.232 | 33.08M | 888.028 | 29.37× | 0.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 97.00M | 100.70M | 1.00× | 1.98M | 2.25M | 1.00× | 96.19M |
| 2 | 175.01M | 167.94M | 1.67× | 2.29M | 2.40M | 1.07× | 96.69M |
| 4 | 284.06M | 350.71M | 3.48× | 2.03M | 2.50M | 1.11× | 94.30M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
