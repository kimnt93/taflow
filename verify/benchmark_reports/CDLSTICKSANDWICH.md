# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.80M | 0.004 | 256.19M | 0.032 | 5.40× | 8.10× |
| 10,000 | 0.046 | 216.05M | 0.042 | 236.09M | 0.089 | 1.92× | 2.09× |
| 100,000 | 0.554 | 180.46M | 0.539 | 185.37M | 0.610 | 1.10× | 1.13× |
| 1,000,000 | 5.976 | 167.34M | 5.724 | 174.70M | 6.306 | 1.06× | 1.10× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.551 ms**; native kernel **0.536 ms**; TA-Lib 0.609 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.362 | 0.290 | 3.45M | 608.973 | 2101.90× | 95.02× |
| 100,000 | 10 | 2.803 | 1.450 | 6.90M | 610.725 | 421.11× | 19.38× |
| 100,000 | 1,000 | 28.866 | 26.227 | 38.13M | 609.074 | 23.22× | 1.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 123.27M | 140.77M | 1.00× | 2.17M | 2.60M | 1.00× | 132.29M |
| 2 | 265.49M | 289.35M | 2.06× | 2.22M | 2.50M | 0.96× | 133.23M |
| 4 | 488.53M | 488.72M | 3.47× | 2.09M | 2.30M | 0.88× | 132.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
