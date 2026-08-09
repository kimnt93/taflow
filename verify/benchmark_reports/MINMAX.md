# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.32M | 0.008 | 118.13M | 0.045 | 4.60× | 5.37× |
| 10,000 | 0.075 | 132.46M | 0.073 | 136.75M | 0.116 | 1.54× | 1.59× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.358 | 0.224 | 4.46M | 44.311 | 197.85× | 162.40× |
| 1,500 | 10 | 1.407 | 0.795 | 12.58M | 44.034 | 55.41× | 44.09× |
| 1,500 | 100 | 5.523 | 3.677 | 27.20M | 45.740 | 12.44× | 9.79× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.96M | 6.60M | 1.00× | 1.17M | 1.37M | 1.00× | 8.18M |
| 2 | 12.65M | 21.83M | 3.31× | 1.10M | 1.39M | 1.02× | 8.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
