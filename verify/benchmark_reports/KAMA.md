# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.80M | 0.006 | 171.71M | 0.036 | 4.90× | 6.15× |
| 10,000 | 0.036 | 275.03M | 0.034 | 296.89M | 0.063 | 1.73× | 1.86× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.314 | 0.178 | 5.61M | 39.285 | 220.34× | 194.55× |
| 1,500 | 10 | 1.226 | 1.869 | 5.35M | 46.021 | 24.62× | 17.20× |
| 1,500 | 100 | 3.648 | 2.177 | 45.93M | 38.683 | 17.77× | 15.03× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.55M | 16.47M | 1.00× | 1.35M | 1.32M | 1.00× | 7.95M |
| 2 | 19.10M | 20.14M | 1.22× | 1.42M | 1.70M | 1.29× | 9.97M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
