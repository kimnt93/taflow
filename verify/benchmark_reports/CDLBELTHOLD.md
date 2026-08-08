# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.85M | 0.005 | 219.41M | 0.037 | 5.75× | 8.09× |
| 10,000 | 0.093 | 107.45M | 0.087 | 114.69M | 0.130 | 1.39× | 1.49× |
| 100,000 | 0.950 | 105.23M | 0.993 | 100.73M | 1.013 | 1.07× | 1.02× |
| 1,000,000 | 10.033 | 99.67M | 9.682 | 103.29M | 10.172 | 1.01× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.961 ms**; native kernel **0.931 ms**; TA-Lib 0.982 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.334 | 0.273 | 3.67M | 1041.129 | 3816.26× | 103.83× |
| 100,000 | 10 | 2.805 | 1.484 | 6.74M | 996.691 | 671.68× | 18.40× |
| 100,000 | 1,000 | 30.041 | 29.620 | 33.76M | 1007.706 | 34.02× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.00M | 92.10M | 1.00× | 2.26M | 1.97M | 1.00× | 88.63M |
| 2 | 174.89M | 191.36M | 2.08× | 2.17M | 2.49M | 1.26× | 85.40M |
| 4 | 294.52M | 306.23M | 3.33× | 2.06M | 2.30M | 1.17× | 83.49M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
