# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.20M | 0.003 | 344.99M | 0.028 | 5.54× | 9.75× |
| 10,000 | 0.021 | 482.56M | 0.016 | 628.21M | 0.042 | 2.01× | 2.61× |
| 100,000 | 0.168 | 593.72M | 0.140 | 714.05M | 0.162 | 0.96× | 1.16× |
| 1,000,000 | 2.883 | 346.83M | 1.983 | 504.34M | 1.914 | 0.66× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.163 ms**; native kernel **0.146 ms**; TA-Lib 0.168 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.438 | 0.287 | 3.48M | 161.192 | 561.04× | 90.61× |
| 100,000 | 10 | 2.661 | 1.183 | 8.46M | 156.049 | 131.95× | 22.20× |
| 100,000 | 1,000 | 6.320 | 3.605 | 277.37M | 169.935 | 47.14× | 7.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 275.17M | 423.59M | 1.00× | 1.96M | 2.78M | 1.00× | 306.31M |
| 2 | 585.68M | 706.13M | 1.67× | 1.93M | 2.58M | 0.93× | 364.36M |
| 4 | 596.49M | 999.21M | 2.36× | 2.01M | 2.30M | 0.83× | 362.99M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
