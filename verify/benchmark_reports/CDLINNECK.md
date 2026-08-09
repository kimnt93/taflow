# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.54M | 0.004 | 252.93M | 0.033 | 5.63× | 8.25× |
| 10,000 | 0.062 | 160.97M | 0.057 | 174.80M | 0.116 | 1.87× | 2.04× |
| 100,000 | 0.872 | 114.64M | 0.840 | 119.02M | 0.896 | 1.03× | 1.07× |
| 1,000,000 | 9.187 | 108.84M | 9.391 | 106.48M | 9.401 | 1.02× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.919 ms**; native kernel **0.844 ms**; TA-Lib 0.900 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.313 | 0.277 | 3.61M | 890.761 | 3214.67× | 104.16× |
| 100,000 | 10 | 2.877 | 1.298 | 7.70M | 929.989 | 716.51× | 21.06× |
| 100,000 | 1,000 | 32.678 | 26.662 | 37.51M | 915.234 | 34.33× | 1.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 90.17M | 97.11M | 1.00× | 2.29M | 2.65M | 1.00× | 94.74M |
| 2 | 183.58M | 191.65M | 1.97× | 2.36M | 2.50M | 0.95× | 92.11M |
| 4 | 315.20M | 345.41M | 3.56× | 2.25M | 2.48M | 0.93× | 91.91M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
