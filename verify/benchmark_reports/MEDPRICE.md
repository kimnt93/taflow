# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 435.99M | 0.001 | 960.86M | 0.028 | 12.13× | 26.72× |
| 10,000 | 0.008 | 1.28G | 0.004 | 2.26G | 0.032 | 4.05× | 7.13× |
| 100,000 | 0.061 | 1.64G | 0.038 | 2.65G | 0.065 | 1.07× | 1.72× |
| 1,000,000 | 1.150 | 869.41M | 0.772 | 1.30G | 0.837 | 0.73× | 1.08× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.061 ms**; native kernel **0.037 ms**; TA-Lib 0.064 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.228 | 0.169 | 5.90M | 64.551 | 380.95× | 154.13× |
| 100,000 | 10 | 1.323 | 0.664 | 15.07M | 63.980 | 96.42× | 39.18× |
| 100,000 | 1,000 | 3.642 | 2.097 | 476.81M | 66.168 | 31.55× | 12.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 396.65M | 788.31M | 1.00× | 2.55M | 3.77M | 1.00× | 534.71M |
| 2 | 846.73M | 1.32G | 1.67× | 2.98M | 3.59M | 0.95× | 691.86M |
| 4 | 903.83M | 1.96G | 2.49× | 3.13M | 3.30M | 0.88× | 634.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
