# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.19M | 0.006 | 181.00M | 0.041 | 5.49× | 7.36× |
| 10,000 | 0.104 | 96.50M | 0.098 | 102.07M | 0.178 | 1.71× | 1.81× |
| 100,000 | 1.243 | 80.45M | 1.209 | 82.68M | 1.487 | 1.20× | 1.23× |
| 1,000,000 | 12.581 | 79.48M | 12.357 | 80.93M | 15.661 | 1.24× | 1.27× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.208 ms**; native kernel **1.180 ms**; TA-Lib 1.538 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.300 | 3.33M | 1478.377 | 4930.01× | 102.58× |
| 100,000 | 10 | 2.895 | 1.525 | 6.56M | 1500.117 | 983.87× | 18.48× |
| 100,000 | 1,000 | 33.451 | 34.070 | 29.35M | 1569.513 | 46.07× | 1.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 75.13M | 77.85M | 1.00× | 1.89M | 2.37M | 1.00× | 64.28M |
| 2 | 139.09M | 153.00M | 1.97× | 2.23M | 2.40M | 1.01× | 58.68M |
| 4 | 240.56M | 282.76M | 3.63× | 2.13M | 2.14M | 0.90× | 59.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
