# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.24M | 0.005 | 204.67M | 0.037 | 5.64× | 7.63× |
| 10,000 | 0.076 | 131.16M | 0.073 | 136.68M | 0.104 | 1.37× | 1.43× |
| 100,000 | 0.849 | 117.81M | 0.847 | 118.02M | 0.818 | 0.96× | 0.97× |
| 1,000,000 | 8.700 | 114.94M | 8.775 | 113.96M | 7.866 | 0.90× | 0.90× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.851 ms**; native kernel **0.841 ms**; TA-Lib 0.819 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.328 | 0.259 | 3.87M | 823.093 | 3183.76× | 119.97× |
| 100,000 | 10 | 2.504 | 1.306 | 7.65M | 812.449 | 621.86× | 24.09× |
| 100,000 | 1,000 | 29.896 | 25.946 | 38.54M | 841.854 | 32.45× | 1.45× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 96.93M | 101.21M | 1.00× | 2.15M | 2.48M | 1.00× | 101.44M |
| 2 | 196.73M | 202.30M | 2.00× | 2.32M | 2.78M | 1.12× | 99.82M |
| 4 | 342.04M | 344.30M | 3.40× | 2.28M | 2.48M | 1.00× | 98.97M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
