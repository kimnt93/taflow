# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.74M | 0.008 | 121.91M | 0.054 | 5.20× | 6.62× |
| 10,000 | 0.061 | 162.60M | 0.054 | 183.51M | 0.246 | 4.00× | 4.51× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.059 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.376 | 0.291 | 3.43M | 56.922 | 195.32× | 107.71× |
| 1,500 | 10 | 2.595 | 1.261 | 7.93M | 57.225 | 45.39× | 23.62× |
| 1,500 | 100 | 5.504 | 3.177 | 31.48M | 58.965 | 18.56× | 9.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.44M | 15.09M | 1.00× | 1.19M | 948.86K | 1.00× | 8.60M |
| 2 | 16.23M | 16.32M | 1.08× | 1.26M | 1.23M | 1.29× | 7.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
