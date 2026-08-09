# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.57M | 0.008 | 118.22M | 0.037 | 3.55× | 4.40× |
| 10,000 | 0.099 | 101.45M | 0.095 | 104.81M | 0.133 | 1.35× | 1.40× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.345 | 0.361 | 2.77M | 40.266 | 111.42× | 79.38× |
| 1,500 | 10 | 3.684 | 1.373 | 7.28M | 40.796 | 29.72× | 21.39× |
| 1,500 | 100 | 5.690 | 3.660 | 27.32M | 39.477 | 10.79× | 7.93× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.10M | 10.31M | 1.00× | 839.79K | 1.20M | 1.00× | 8.52M |
| 2 | 12.64M | 17.81M | 1.73× | 1.37M | 1.41M | 1.17× | 9.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
