# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.05M | 0.010 | 102.66M | 0.043 | 3.76× | 4.38× |
| 10,000 | 0.083 | 121.09M | 0.081 | 124.20M | 0.173 | 2.09× | 2.14× |
| 100,000 | 0.790 | 126.61M | 0.765 | 130.79M | 1.492 | 1.89× | 1.95× |
| 1,000,000 | 8.474 | 118.00M | 8.417 | 118.81M | 14.543 | 1.72× | 1.73× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.793 ms**; native kernel **0.773 ms**; TA-Lib 1.499 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.340 | 0.285 | 3.51M | 1438.182 | 5053.86× | 95.40× |
| 100,000 | 10 | 2.537 | 1.292 | 7.74M | 1425.017 | 1103.22× | 21.55× |
| 100,000 | 1,000 | 30.439 | 23.055 | 43.37M | 1464.387 | 63.52× | 1.76× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.03M | 103.83M | 1.00× | 2.53M | 1.99M | 1.00× | 58.09M |
| 2 | 193.37M | 203.69M | 1.96× | 2.33M | 2.52M | 1.26× | 59.36M |
| 4 | 342.94M | 391.68M | 3.77× | 2.30M | 2.55M | 1.28× | 59.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
