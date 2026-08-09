# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 274.92M | 0.002 | 549.19M | 0.029 | 7.87× | 15.71× |
| 10,000 | 0.011 | 890.70M | 0.007 | 1.34G | 0.035 | 3.15× | 4.73× |
| 100,000 | 0.085 | 1.18G | 0.059 | 1.70G | 0.087 | 1.03× | 1.48× |
| 1,000,000 | 1.854 | 539.32M | 1.461 | 684.45M | 1.467 | 0.79× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.086 ms**; native kernel **0.059 ms**; TA-Lib 0.088 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.297 | 0.235 | 4.25M | 86.723 | 368.48× | 112.31× |
| 100,000 | 10 | 2.228 | 1.093 | 9.15M | 85.848 | 78.56× | 24.65× |
| 100,000 | 1,000 | 5.132 | 2.859 | 349.77M | 86.795 | 30.36× | 10.41× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 399.94M | 451.48M | 1.00× | 2.60M | 2.95M | 1.00× | 442.79M |
| 2 | 857.99M | 1.15G | 2.55× | 2.60M | 2.96M | 1.00× | 573.68M |
| 4 | 857.91M | 1.64G | 3.63× | 2.45M | 2.61M | 0.88× | 520.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
