# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 397.10M | 0.001 | 934.11M | 0.029 | 11.58× | 27.24× |
| 10,000 | 0.007 | 1.34G | 0.004 | 2.42G | 0.034 | 4.51× | 8.16× |
| 100,000 | 0.067 | 1.50G | 0.040 | 2.52G | 0.069 | 1.03× | 1.73× |
| 1,000,000 | 1.152 | 868.24M | 0.804 | 1.24G | 0.840 | 0.73× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.064 ms**; native kernel **0.040 ms**; TA-Lib 0.069 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.163 | 6.13M | 67.781 | 415.66× | 170.61× |
| 100,000 | 10 | 1.305 | 0.686 | 14.59M | 70.268 | 102.49× | 41.42× |
| 100,000 | 1,000 | 3.638 | 2.178 | 459.23M | 68.711 | 31.55× | 13.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 485.74M | 965.46M | 1.00× | 2.83M | 3.46M | 1.00× | 642.01M |
| 2 | 712.75M | 1.57G | 1.63× | 2.66M | 3.64M | 1.05× | 729.89M |
| 4 | 896.81M | 2.14G | 2.21× | 2.63M | 3.27M | 0.94× | 658.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
