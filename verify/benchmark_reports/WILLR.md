# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.31M | 0.008 | 132.75M | 0.035 | 3.91× | 4.71× |
| 10,000 | 0.061 | 164.45M | 0.055 | 180.28M | 0.116 | 1.91× | 2.10× |
| 100,000 | 0.562 | 177.82M | 0.539 | 185.53M | 0.852 | 1.52× | 1.58× |
| 1,000,000 | 7.106 | 140.72M | 6.413 | 155.94M | 8.688 | 1.22× | 1.35× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.572 ms**; native kernel **0.531 ms**; TA-Lib 0.842 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.373 | 0.272 | 3.68M | 824.677 | 3037.26× | 107.82× |
| 100,000 | 10 | 2.267 | 1.297 | 7.71M | 856.997 | 661.00× | 22.78× |
| 100,000 | 1,000 | 29.592 | 25.329 | 39.48M | 860.755 | 33.98× | 1.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.78M | 146.29M | 1.00× | 2.02M | 2.11M | 1.00× | 104.31M |
| 2 | 274.94M | 269.45M | 1.84× | 1.75M | 2.72M | 1.29× | 102.24M |
| 4 | 385.04M | 447.78M | 3.06× | 2.11M | 2.51M | 1.19× | 103.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
