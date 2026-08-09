# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.28M | 0.007 | 140.18M | 0.034 | 3.72× | 4.77× |
| 10,000 | 0.049 | 205.72M | 0.046 | 217.85M | 0.086 | 1.77× | 1.88× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.336 | 0.261 | 3.83M | 36.586 | 140.21× | 109.59× |
| 1,500 | 10 | 3.972 | 1.249 | 8.01M | 36.232 | 29.01× | 22.50× |
| 1,500 | 100 | 5.129 | 2.940 | 34.01M | 36.351 | 12.36× | 10.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.42M | 9.77M | 1.00× | 1.19M | 1.24M | 1.00× | 7.77M |
| 2 | 17.31M | 18.69M | 1.91× | 1.34M | 1.42M | 1.14× | 10.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
