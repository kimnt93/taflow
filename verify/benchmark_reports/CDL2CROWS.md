# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.85M | 0.004 | 279.68M | 0.030 | 5.63× | 8.51× |
| 10,000 | 0.061 | 163.29M | 0.055 | 180.63M | 0.104 | 1.70× | 1.88× |
| 100,000 | 0.895 | 111.75M | 0.865 | 115.58M | 0.854 | 0.95× | 0.99× |
| 1,000,000 | 8.962 | 111.59M | 8.740 | 114.42M | 8.346 | 0.93× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.884 ms**; native kernel **0.860 ms**; TA-Lib 0.842 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.337 | 0.273 | 3.67M | 844.761 | 3099.64× | 99.78× |
| 100,000 | 10 | 2.513 | 1.341 | 7.46M | 877.693 | 654.44× | 20.55× |
| 100,000 | 1,000 | 28.960 | 26.167 | 38.22M | 853.001 | 32.60× | 1.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 91.29M | 98.00M | 1.00× | 2.30M | 2.16M | 1.00× | 102.16M |
| 2 | 188.06M | 179.99M | 1.84× | 2.41M | 2.76M | 1.28× | 100.80M |
| 4 | 335.43M | 365.15M | 3.73× | 2.36M | 2.53M | 1.17× | 98.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
