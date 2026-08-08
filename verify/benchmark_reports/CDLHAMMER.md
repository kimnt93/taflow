# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.23M | 0.005 | 187.64M | 0.041 | 5.45× | 7.68× |
| 10,000 | 0.103 | 97.05M | 0.101 | 99.41M | 0.181 | 1.76× | 1.80× |
| 100,000 | 1.208 | 82.81M | 1.245 | 80.30M | 1.523 | 1.26× | 1.22× |
| 1,000,000 | 12.524 | 79.85M | 12.207 | 81.92M | 15.316 | 1.22× | 1.25× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.205 ms**; native kernel **1.185 ms**; TA-Lib 1.628 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.354 | 0.291 | 3.44M | 1738.052 | 5973.89× | 96.85× |
| 100,000 | 10 | 2.645 | 1.370 | 7.30M | 1523.181 | 1111.88× | 20.25× |
| 100,000 | 1,000 | 28.579 | 24.373 | 41.03M | 1460.428 | 59.92× | 1.71× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 69.37M | 72.76M | 1.00× | 2.11M | 2.15M | 1.00× | 58.38M |
| 2 | 133.20M | 147.20M | 2.02× | 2.03M | 2.59M | 1.20× | 59.54M |
| 4 | 228.94M | 263.13M | 3.62× | 2.20M | 2.48M | 1.15× | 59.71M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
