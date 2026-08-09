# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.91M | 0.005 | 192.22M | 0.038 | 5.45× | 7.34× |
| 10,000 | 0.100 | 100.24M | 0.096 | 103.90M | 0.135 | 1.35× | 1.40× |
| 100,000 | 1.037 | 96.43M | 1.021 | 97.98M | 1.039 | 1.00× | 1.02× |
| 1,000,000 | 10.613 | 94.22M | 10.467 | 95.53M | 10.293 | 0.97× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.032 ms**; native kernel **1.012 ms**; TA-Lib 1.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.277 | 3.61M | 1039.264 | 3750.64× | 112.81× |
| 100,000 | 10 | 2.589 | 1.356 | 7.38M | 1032.166 | 761.33× | 23.23× |
| 100,000 | 1,000 | 31.156 | 27.996 | 35.72M | 1046.214 | 37.37× | 1.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.93M | 90.44M | 1.00× | 2.19M | 2.68M | 1.00× | 84.23M |
| 2 | 162.32M | 169.63M | 1.88× | 2.34M | 2.64M | 0.98× | 83.62M |
| 4 | 288.62M | 313.72M | 3.47× | 2.35M | 2.50M | 0.93× | 84.14M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
