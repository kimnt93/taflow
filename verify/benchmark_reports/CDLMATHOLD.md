# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.79M | 0.006 | 162.67M | 0.038 | 4.82× | 6.19× |
| 10,000 | 0.100 | 100.13M | 0.096 | 103.89M | 0.121 | 1.21× | 1.25× |
| 100,000 | 1.026 | 97.43M | 0.999 | 100.13M | 0.893 | 0.87× | 0.89× |
| 1,000,000 | 10.544 | 94.84M | 10.134 | 98.68M | 8.644 | 0.82× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.120 ms**; native kernel **1.009 ms**; TA-Lib 0.895 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.370 | 0.305 | 3.28M | 903.287 | 2965.72× | 110.54× |
| 100,000 | 10 | 2.808 | 1.595 | 6.27M | 892.434 | 559.51× | 20.71× |
| 100,000 | 1,000 | 40.748 | 35.029 | 28.55M | 1081.297 | 30.87× | 1.12× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 79.67M | 92.41M | 1.00× | 2.21M | 2.51M | 1.00× | 96.46M |
| 2 | 169.06M | 171.77M | 1.86× | 2.24M | 2.35M | 0.93× | 98.66M |
| 4 | 301.38M | 338.36M | 3.66× | 2.22M | 2.48M | 0.99× | 98.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
