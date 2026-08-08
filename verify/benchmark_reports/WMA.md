# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.99M | 0.004 | 227.90M | 0.033 | 5.79× | 7.59× |
| 10,000 | 0.044 | 226.59M | 0.038 | 264.89M | 0.051 | 1.16× | 1.36× |
| 100,000 | 0.353 | 283.29M | 0.310 | 322.73M | 0.223 | 0.63× | 0.72× |
| 1,000,000 | 4.121 | 242.69M | 3.664 | 272.92M | 2.049 | 0.50× | 0.56× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.341 ms**; native kernel **0.313 ms**; TA-Lib 0.224 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.163 | 6.13M | 225.401 | 1382.01× | 186.01× |
| 100,000 | 10 | 0.941 | 0.642 | 15.57M | 239.167 | 372.49× | 47.40× |
| 100,000 | 1,000 | 5.771 | 4.397 | 227.44M | 228.465 | 51.96× | 7.76× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 213.28M | 248.14M | 1.00× | 3.22M | 3.15M | 1.00× | 289.35M |
| 2 | 389.70M | 494.07M | 1.99× | 2.94M | 3.41M | 1.08× | 307.45M |
| 4 | 510.27M | 860.66M | 3.47× | 2.98M | 3.32M | 1.05× | 303.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
