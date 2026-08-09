# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.38M | 0.008 | 126.41M | 0.036 | 3.82× | 4.54× |
| 10,000 | 0.092 | 108.81M | 0.090 | 110.86M | 0.126 | 1.37× | 1.40× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.346 | 0.270 | 3.70M | 40.945 | 151.39× | 107.13× |
| 1,500 | 10 | 3.821 | 1.294 | 7.73M | 42.215 | 32.63× | 21.08× |
| 1,500 | 100 | 5.626 | 3.170 | 31.54M | 43.229 | 13.64× | 9.12× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.50M | 12.46M | 1.00× | 884.63K | 1.33M | 1.00× | 8.61M |
| 2 | 19.38M | 11.95M | 0.96× | 1.36M | 1.33M | 1.00× | 8.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
