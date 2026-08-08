# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.04M | 0.004 | 222.24M | 0.032 | 4.97× | 7.22× |
| 10,000 | 0.061 | 163.50M | 0.056 | 178.21M | 0.080 | 1.31× | 1.43× |
| 100,000 | 0.560 | 178.57M | 0.557 | 179.45M | 0.547 | 0.98× | 0.98× |
| 1,000,000 | 5.826 | 171.63M | 5.754 | 173.81M | 5.351 | 0.92× | 0.93× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.553 ms**; native kernel **0.548 ms**; TA-Lib 0.556 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.251 | 3.99M | 554.744 | 2213.66× | 108.88× |
| 100,000 | 10 | 2.410 | 1.202 | 8.32M | 554.551 | 461.48× | 22.78× |
| 100,000 | 1,000 | 9.855 | 7.854 | 127.32M | 565.354 | 71.98× | 4.16× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.67M | 147.17M | 1.00× | 2.36M | 2.46M | 1.00× | 143.79M |
| 2 | 268.58M | 289.03M | 1.96× | 2.45M | 2.67M | 1.09× | 141.56M |
| 4 | 403.16M | 464.42M | 3.16× | 2.30M | 2.42M | 0.99× | 142.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
