# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.85M | 0.006 | 164.55M | 0.048 | 5.99× | 7.84× |
| 10,000 | 0.088 | 114.26M | 0.084 | 119.73M | 0.227 | 2.59× | 2.72× |
| 100,000 | 0.863 | 115.94M | 0.849 | 117.81M | 1.938 | 2.25× | 2.28× |
| 1,000,000 | 9.023 | 110.83M | 8.980 | 111.36M | 19.262 | 2.13× | 2.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.863 ms**; native kernel **0.850 ms**; TA-Lib 1.920 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.357 | 0.281 | 3.56M | 1916.340 | 6821.85× | 98.21× |
| 100,000 | 10 | 2.565 | 1.360 | 7.35M | 1926.504 | 1416.67× | 20.45× |
| 100,000 | 1,000 | 30.192 | 27.471 | 36.40M | 1929.193 | 70.23× | 1.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 97.34M | 91.51M | 1.00× | 2.26M | 2.33M | 1.00× | 47.34M |
| 2 | 190.03M | 185.87M | 2.03× | 2.29M | 2.65M | 1.14× | 47.90M |
| 4 | 343.84M | 355.69M | 3.89× | 2.08M | 2.47M | 1.06× | 47.65M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
