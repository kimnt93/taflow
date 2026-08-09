# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.64M | 0.011 | 90.75M | 0.039 | 3.35× | 3.55× |
| 10,000 | 0.084 | 119.19M | 0.087 | 114.49M | 0.118 | 1.41× | 1.35× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.016 ms**; native kernel **0.015 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.311 | 0.241 | 4.15M | 43.473 | 180.28× | 129.20× |
| 1,500 | 10 | 1.225 | 1.093 | 9.15M | 44.353 | 40.59× | 28.85× |
| 1,500 | 100 | 3.566 | 2.969 | 33.68M | 44.129 | 14.86× | 10.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.17M | 14.82M | 1.00× | 1.35M | 1.43M | 1.00× | 8.73M |
| 2 | 18.84M | 16.77M | 1.13× | 1.43M | 1.31M | 0.92× | 7.82M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
