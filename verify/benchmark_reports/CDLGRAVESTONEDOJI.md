# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.31M | 0.004 | 272.34M | 0.035 | 6.22× | 9.45× |
| 10,000 | 0.046 | 217.71M | 0.042 | 235.67M | 0.104 | 2.26× | 2.44× |
| 100,000 | 0.562 | 178.03M | 0.542 | 184.34M | 0.754 | 1.34× | 1.39× |
| 1,000,000 | 6.381 | 156.73M | 6.135 | 163.01M | 7.468 | 1.17× | 1.22× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.552 ms**; native kernel **0.560 ms**; TA-Lib 0.762 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.339 | 0.275 | 3.63M | 737.813 | 2679.87× | 104.69× |
| 100,000 | 10 | 2.639 | 1.527 | 6.55M | 740.595 | 485.08× | 18.25× |
| 100,000 | 1,000 | 31.738 | 26.974 | 37.07M | 774.511 | 28.71× | 1.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 140.15M | 130.44M | 1.00× | 2.45M | 2.33M | 1.00× | 109.55M |
| 2 | 254.01M | 282.39M | 2.16× | 2.45M | 2.54M | 1.09× | 106.85M |
| 4 | 432.87M | 439.59M | 3.37× | 2.35M | 2.44M | 1.04× | 107.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
