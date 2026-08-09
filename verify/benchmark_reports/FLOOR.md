# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 232.91M | 0.003 | 306.87M | 0.027 | 6.22× | 8.19× |
| 10,000 | 0.030 | 335.65M | 0.025 | 405.16M | 0.043 | 1.44× | 1.74× |
| 100,000 | 0.273 | 366.95M | 0.251 | 398.63M | 0.156 | 0.57× | 0.62× |
| 1,000,000 | 3.557 | 281.10M | 3.137 | 318.78M | 1.469 | 0.41× | 0.47× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.273 ms**; native kernel **0.251 ms**; TA-Lib 0.158 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.157 | 6.36M | 157.233 | 1000.63× | 154.00× |
| 100,000 | 10 | 0.864 | 0.579 | 17.28M | 155.917 | 269.48× | 43.03× |
| 100,000 | 1,000 | 5.068 | 3.971 | 251.80M | 158.537 | 39.92× | 6.78× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 212.33M | 272.20M | 1.00× | 2.63M | 3.03M | 1.00× | 367.41M |
| 2 | 395.76M | 531.97M | 1.95× | 3.16M | 3.50M | 1.16× | 358.88M |
| 4 | 404.78M | 653.78M | 2.40× | 3.19M | 3.41M | 1.13× | 415.55M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
