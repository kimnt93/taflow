# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.25M | 0.015 | 67.87M | 0.045 | 2.83× | 3.03× |
| 10,000 | 0.134 | 74.61M | 0.130 | 77.05M | 0.140 | 1.04× | 1.08× |
| 100,000 | 1.285 | 77.80M | 1.232 | 81.20M | 1.048 | 0.82× | 0.85× |
| 1,000,000 | 14.401 | 69.44M | 13.329 | 75.02M | 10.942 | 0.76× | 0.82× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.294 ms**; native kernel **1.238 ms**; TA-Lib 1.031 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.491 | 0.360 | 2.78M | 1036.080 | 2878.56× | 107.81× |
| 100,000 | 10 | 2.370 | 2.058 | 4.86M | 1026.000 | 498.64× | 18.61× |
| 100,000 | 1,000 | 101.955 | 88.251 | 11.33M | 1032.256 | 11.70× | 0.51× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.93M | 72.02M | 1.00× | 1.82M | 1.81M | 1.00× | 78.69M |
| 2 | 126.64M | 139.05M | 1.93× | 1.75M | 2.03M | 1.12× | 80.02M |
| 4 | 174.23M | 219.27M | 3.04× | 1.84M | 1.87M | 1.03× | 80.59M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
