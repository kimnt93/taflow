# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 181.64M | 0.004 | 271.23M | 0.034 | 6.13× | 9.15× |
| 10,000 | 0.048 | 206.78M | 0.045 | 222.79M | 0.085 | 1.75× | 1.88× |
| 100,000 | 0.549 | 182.08M | 0.524 | 190.71M | 0.595 | 1.08× | 1.14× |
| 1,000,000 | 5.976 | 167.33M | 5.799 | 172.45M | 6.034 | 1.01× | 1.04× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.551 ms**; native kernel **0.538 ms**; TA-Lib 0.603 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.254 | 3.94M | 589.863 | 2326.73× | 112.39× |
| 100,000 | 10 | 2.469 | 1.307 | 7.65M | 588.014 | 450.02× | 21.60× |
| 100,000 | 1,000 | 25.327 | 22.295 | 44.85M | 595.297 | 26.70× | 1.51× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 118.04M | 140.72M | 1.00× | 2.15M | 2.59M | 1.00× | 129.63M |
| 2 | 277.52M | 271.37M | 1.93× | 2.37M | 2.64M | 1.02× | 130.97M |
| 4 | 464.54M | 490.11M | 3.48× | 2.48M | 2.75M | 1.06× | 135.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
