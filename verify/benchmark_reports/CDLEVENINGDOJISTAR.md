# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.52M | 0.005 | 203.25M | 0.039 | 5.81× | 7.89× |
| 10,000 | 0.081 | 123.30M | 0.077 | 129.34M | 0.117 | 1.44× | 1.51× |
| 100,000 | 0.882 | 113.34M | 0.872 | 114.72M | 0.867 | 0.98× | 0.99× |
| 1,000,000 | 9.328 | 107.20M | 9.188 | 108.83M | 8.587 | 0.92× | 0.93× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.881 ms**; native kernel **0.873 ms**; TA-Lib 0.862 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.333 | 0.270 | 3.71M | 872.693 | 3237.65× | 117.68× |
| 100,000 | 10 | 2.546 | 1.342 | 7.45M | 868.655 | 647.21× | 23.63× |
| 100,000 | 1,000 | 29.848 | 26.728 | 37.41M | 867.967 | 32.47× | 1.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.73M | 91.81M | 1.00× | 2.09M | 2.17M | 1.00× | 99.28M |
| 2 | 191.25M | 200.85M | 2.19× | 2.41M | 2.64M | 1.22× | 98.42M |
| 4 | 323.56M | 360.33M | 3.92× | 2.21M | 2.45M | 1.13× | 97.97M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
