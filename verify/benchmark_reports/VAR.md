# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.66M | 0.006 | 180.08M | 0.036 | 5.38× | 6.51× |
| 10,000 | 0.037 | 266.71M | 0.034 | 290.95M | 0.057 | 1.51× | 1.65× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.008 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.296 | 0.166 | 6.02M | 40.437 | 243.40× | 186.87× |
| 1,500 | 10 | 1.165 | 0.633 | 15.79M | 34.745 | 54.86× | 52.60× |
| 1,500 | 100 | 3.070 | 2.011 | 49.72M | 34.453 | 17.13× | 16.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.90M | 17.21M | 1.00× | 1.19M | 1.48M | 1.00× | 8.31M |
| 2 | 19.70M | 15.43M | 0.90× | 1.27M | 1.71M | 1.16× | 9.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
