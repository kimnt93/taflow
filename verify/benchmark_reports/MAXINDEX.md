# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.90M | 0.005 | 193.94M | 0.038 | 6.07× | 7.37× |
| 10,000 | 0.052 | 193.83M | 0.049 | 204.98M | 0.094 | 1.83× | 1.93× |
| 100,000 | 0.504 | 198.40M | 0.488 | 204.94M | 0.670 | 1.33× | 1.37× |
| 1,000,000 | 5.311 | 188.28M | 5.008 | 199.69M | 6.633 | 1.25× | 1.32× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.523 ms**; native kernel **0.491 ms**; TA-Lib 0.681 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.159 | 6.28M | 682.530 | 4288.04× | 182.62× |
| 100,000 | 10 | 0.953 | 0.583 | 17.16M | 666.518 | 1143.94× | 49.53× |
| 100,000 | 1,000 | 14.460 | 12.257 | 81.59M | 671.028 | 54.75× | 2.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 114.73M | 157.96M | 1.00× | 2.78M | 3.33M | 1.00× | 117.84M |
| 2 | 244.39M | 308.98M | 1.96× | 3.18M | 4.00M | 1.20× | 121.18M |
| 4 | 396.28M | 527.12M | 3.34× | 3.09M | 3.37M | 1.01× | 119.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
