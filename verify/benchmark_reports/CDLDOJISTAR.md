# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.08M | 0.005 | 211.21M | 0.034 | 5.24× | 7.23× |
| 10,000 | 0.099 | 101.48M | 0.096 | 104.42M | 0.130 | 1.32× | 1.36× |
| 100,000 | 1.046 | 95.64M | 1.023 | 97.79M | 1.057 | 1.01× | 1.03× |
| 1,000,000 | 11.072 | 90.32M | 10.771 | 92.84M | 10.479 | 0.95× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.041 ms**; native kernel **1.025 ms**; TA-Lib 1.047 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.276 | 3.62M | 1044.305 | 3784.31× | 102.39× |
| 100,000 | 10 | 2.570 | 1.394 | 7.18M | 1042.078 | 747.78× | 19.67× |
| 100,000 | 1,000 | 31.640 | 28.235 | 35.42M | 1058.728 | 37.50× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 84.75M | 83.73M | 1.00× | 2.10M | 2.78M | 1.00× | 82.34M |
| 2 | 167.88M | 161.65M | 1.93× | 2.30M | 2.54M | 0.91× | 86.22M |
| 4 | 261.50M | 310.02M | 3.70× | 2.32M | 2.41M | 0.87× | 82.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
