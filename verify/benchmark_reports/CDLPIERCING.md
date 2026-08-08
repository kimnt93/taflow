# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.92M | 0.004 | 257.70M | 0.033 | 5.48× | 8.41× |
| 10,000 | 0.072 | 139.48M | 0.081 | 124.18M | 0.136 | 1.89× | 1.68× |
| 100,000 | 0.877 | 114.05M | 1.007 | 99.26M | 1.184 | 1.35× | 1.18× |
| 1,000,000 | 9.368 | 106.74M | 8.875 | 112.68M | 10.063 | 1.07× | 1.13× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.818 ms**; native kernel **0.800 ms**; TA-Lib 1.015 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.352 | 0.286 | 3.50M | 1034.275 | 3619.93× | 106.26× |
| 100,000 | 10 | 3.794 | 1.675 | 5.97M | 1096.809 | 654.87× | 17.77× |
| 100,000 | 1,000 | 30.351 | 28.668 | 34.88M | 1063.115 | 37.08× | 1.13× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 96.92M | 99.02M | 1.00× | 2.34M | 2.08M | 1.00× | 84.88M |
| 2 | 197.66M | 209.42M | 2.11× | 2.26M | 2.55M | 1.23× | 88.93M |
| 4 | 337.54M | 305.23M | 3.08× | 2.14M | 2.47M | 1.19× | 84.48M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
