# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.44M | 0.013 | 74.12M | 0.050 | 3.24× | 3.67× |
| 10,000 | 0.091 | 109.94M | 0.138 | 72.26M | 0.180 | 1.97× | 1.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.017 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.345 | 0.270 | 3.71M | 51.599 | 191.31× | 109.47× |
| 1,500 | 10 | 2.694 | 1.368 | 7.31M | 49.805 | 36.40× | 20.80× |
| 1,500 | 100 | 5.863 | 3.448 | 29.01M | 50.232 | 14.57× | 8.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.34M | 9.29M | 1.00× | 978.77K | 1.20M | 1.00× | 8.33M |
| 2 | 16.53M | 18.08M | 1.95× | 1.31M | 1.36M | 1.13× | 8.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
