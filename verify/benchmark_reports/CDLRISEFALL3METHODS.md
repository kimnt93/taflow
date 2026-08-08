# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.54M | 0.016 | 63.82M | 0.034 | 1.87× | 2.15× |
| 10,000 | 0.167 | 59.88M | 0.174 | 57.55M | 0.111 | 0.67× | 0.64× |
| 100,000 | 1.667 | 59.99M | 1.668 | 59.96M | 0.885 | 0.53× | 0.53× |
| 1,000,000 | 17.231 | 58.03M | 17.246 | 57.99M | 9.980 | 0.58× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.658 ms**; native kernel **1.730 ms**; TA-Lib 0.915 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.331 | 0.255 | 3.92M | 903.439 | 3542.74× | 106.20× |
| 100,000 | 10 | 2.363 | 1.184 | 8.45M | 954.346 | 806.06× | 23.48× |
| 100,000 | 1,000 | 20.687 | 21.923 | 45.61M | 960.577 | 43.82× | 1.63× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.43M | 46.20M | 1.00× | 1.67M | 1.97M | 1.00× | 78.48M |
| 2 | 49.93M | 52.21M | 1.13× | 2.09M | 2.30M | 1.17× | 83.31M |
| 4 | 51.78M | 50.97M | 1.10× | 2.12M | 2.16M | 1.10× | 76.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
