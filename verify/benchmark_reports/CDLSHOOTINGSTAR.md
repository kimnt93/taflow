# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.65M | 0.005 | 202.13M | 0.041 | 5.76× | 8.27× |
| 10,000 | 0.097 | 102.97M | 0.099 | 100.80M | 0.173 | 1.78× | 1.74× |
| 100,000 | 1.195 | 83.68M | 1.144 | 87.40M | 1.443 | 1.21× | 1.26× |
| 1,000,000 | 12.127 | 82.46M | 11.989 | 83.41M | 14.080 | 1.16× | 1.17× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.186 ms**; native kernel **1.143 ms**; TA-Lib 1.404 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.365 | 0.291 | 3.44M | 1395.120 | 4794.32× | 93.85× |
| 100,000 | 10 | 2.687 | 1.404 | 7.12M | 1388.465 | 989.16× | 20.05× |
| 100,000 | 1,000 | 30.389 | 33.833 | 29.56M | 1413.769 | 41.79× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.12M | 74.82M | 1.00× | 2.06M | 1.99M | 1.00× | 64.76M |
| 2 | 136.89M | 151.94M | 2.03× | 2.33M | 2.56M | 1.29× | 65.41M |
| 4 | 247.98M | 229.74M | 3.07× | 2.15M | 2.39M | 1.20× | 62.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
