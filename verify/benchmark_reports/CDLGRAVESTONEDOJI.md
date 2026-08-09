# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.56M | 0.004 | 266.46M | 0.034 | 5.95× | 8.99× |
| 10,000 | 0.051 | 194.28M | 0.048 | 208.03M | 0.098 | 1.89× | 2.03× |
| 100,000 | 0.568 | 176.21M | 0.554 | 180.54M | 0.738 | 1.30× | 1.33× |
| 1,000,000 | 6.017 | 166.20M | 5.895 | 169.62M | 7.518 | 1.25× | 1.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.568 ms**; native kernel **0.554 ms**; TA-Lib 0.735 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.355 | 0.279 | 3.58M | 733.044 | 2623.68× | 95.98× |
| 100,000 | 10 | 2.443 | 1.264 | 7.91M | 736.513 | 582.67× | 21.71× |
| 100,000 | 1,000 | 25.730 | 20.901 | 47.85M | 751.554 | 35.96× | 1.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.74M | 139.37M | 1.00× | 2.17M | 2.46M | 1.00× | 116.84M |
| 2 | 278.99M | 277.83M | 1.99× | 2.40M | 2.62M | 1.06× | 111.96M |
| 4 | 458.49M | 517.71M | 3.71× | 2.33M | 2.55M | 1.03× | 116.10M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
