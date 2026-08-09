# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 358.62M | 0.001 | 753.19M | 0.028 | 10.09× | 21.20× |
| 10,000 | 0.010 | 1.02G | 0.006 | 1.68G | 0.034 | 3.43× | 5.69× |
| 100,000 | 0.077 | 1.30G | 0.051 | 1.95G | 0.081 | 1.05× | 1.57× |
| 1,000,000 | 1.492 | 670.32M | 1.114 | 897.28M | 1.161 | 0.78× | 1.04× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.076 ms**; native kernel **0.051 ms**; TA-Lib 0.080 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.219 | 4.56M | 81.267 | 370.82× | 121.65× |
| 100,000 | 10 | 1.843 | 0.958 | 10.44M | 81.352 | 84.92× | 28.09× |
| 100,000 | 1,000 | 4.370 | 2.476 | 403.83M | 84.598 | 34.16× | 11.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 406.72M | 716.75M | 1.00× | 2.76M | 2.95M | 1.00× | 620.89M |
| 2 | 823.46M | 1.33G | 1.85× | 2.74M | 3.04M | 1.03× | 587.38M |
| 4 | 887.35M | 1.66G | 2.31× | 2.79M | 2.92M | 0.99× | 605.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
