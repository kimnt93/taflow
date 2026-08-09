# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.71M | 0.007 | 134.88M | 0.032 | 3.65× | 4.29× |
| 10,000 | 0.061 | 163.67M | 0.057 | 176.41M | 0.077 | 1.25× | 1.35× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.008 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.343 | 0.331 | 3.02M | 31.909 | 96.27× | 83.69× |
| 1,500 | 10 | 2.535 | 1.245 | 8.03M | 31.493 | 25.29× | 22.42× |
| 1,500 | 100 | 5.469 | 3.371 | 29.66M | 33.551 | 9.95× | 8.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.45M | 16.13M | 1.00× | 1.20M | 1.17M | 1.00× | 8.63M |
| 2 | 15.46M | 19.51M | 1.21× | 1.01M | 1.30M | 1.11× | 9.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
