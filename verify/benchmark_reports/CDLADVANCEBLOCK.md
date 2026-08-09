# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.49M | 0.009 | 112.62M | 0.046 | 4.29× | 5.17× |
| 10,000 | 0.093 | 108.09M | 0.086 | 115.72M | 0.296 | 3.20× | 3.42× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.014 ms**; TA-Lib 0.060 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.392 | 0.297 | 3.36M | 61.163 | 205.76× | 95.43× |
| 1,500 | 10 | 2.570 | 1.742 | 5.74M | 55.711 | 31.99× | 16.29× |
| 1,500 | 100 | 5.977 | 3.449 | 29.00M | 58.483 | 16.96× | 8.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.02M | 8.72M | 1.00× | 1.21M | 1.14M | 1.00× | 7.40M |
| 2 | 15.14M | 19.36M | 2.22× | 1.24M | 1.36M | 1.19× | 8.48M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
