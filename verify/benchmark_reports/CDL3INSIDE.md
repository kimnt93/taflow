# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.49M | 0.005 | 197.13M | 0.038 | 5.62× | 7.56× |
| 10,000 | 0.104 | 96.58M | 0.101 | 98.62M | 0.135 | 1.31× | 1.33× |
| 100,000 | 1.029 | 97.15M | 1.041 | 96.02M | 1.056 | 1.03× | 1.01× |
| 1,000,000 | 10.809 | 92.52M | 11.252 | 88.87M | 10.942 | 1.01× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.055 ms**; native kernel **1.009 ms**; TA-Lib 1.107 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.377 | 0.297 | 3.37M | 1062.324 | 3575.33× | 98.95× |
| 100,000 | 10 | 3.004 | 1.490 | 6.71M | 1070.561 | 718.72× | 18.65× |
| 100,000 | 1,000 | 34.903 | 32.267 | 30.99M | 1063.749 | 32.97× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 85.97M | 85.03M | 1.00× | 2.06M | 2.56M | 1.00× | 79.82M |
| 2 | 171.79M | 166.88M | 1.96× | 2.36M | 2.61M | 1.02× | 80.16M |
| 4 | 283.98M | 301.57M | 3.55× | 2.15M | 2.29M | 0.90× | 82.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
