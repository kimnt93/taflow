# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.51M | 0.004 | 239.25M | 0.032 | 5.21× | 7.63× |
| 10,000 | 0.065 | 153.61M | 0.058 | 172.19M | 0.081 | 1.24× | 1.39× |
| 100,000 | 0.921 | 108.58M | 0.920 | 108.65M | 0.582 | 0.63× | 0.63× |
| 1,000,000 | 9.568 | 104.52M | 9.436 | 105.98M | 5.904 | 0.62× | 0.63× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.908 ms**; native kernel **0.906 ms**; TA-Lib 0.588 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.407 | 0.277 | 3.60M | 602.565 | 2172.05× | 102.53× |
| 100,000 | 10 | 2.906 | 1.362 | 7.34M | 603.616 | 443.07× | 20.29× |
| 100,000 | 1,000 | 29.185 | 26.759 | 37.37M | 590.983 | 22.09× | 1.14× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.97M | 97.41M | 1.00× | 1.98M | 2.50M | 1.00× | 141.19M |
| 2 | 184.18M | 182.22M | 1.87× | 2.43M | 2.48M | 1.00× | 142.86M |
| 4 | 332.95M | 333.47M | 3.42× | 2.43M | 2.52M | 1.01× | 145.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
