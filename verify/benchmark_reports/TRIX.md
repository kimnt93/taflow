# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.85M | 0.005 | 198.83M | 0.040 | 5.88× | 7.86× |
| 10,000 | 0.041 | 242.74M | 0.037 | 269.57M | 0.126 | 3.05× | 3.38× |
| 100,000 | 0.380 | 263.32M | 0.352 | 283.97M | 0.977 | 2.57× | 2.77× |
| 1,000,000 | 4.158 | 240.49M | 3.576 | 279.63M | 9.406 | 2.26× | 2.63× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.381 ms**; native kernel **0.358 ms**; TA-Lib 1.018 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.151 | 6.60M | 991.016 | 6545.20× | 215.63× |
| 100,000 | 10 | 1.116 | 0.608 | 16.44M | 996.380 | 1638.24× | 51.55× |
| 100,000 | 1,000 | 6.013 | 5.554 | 180.06M | 941.238 | 169.48× | 7.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 189.88M | 227.87M | 1.00× | 3.49M | 3.89M | 1.00× | 94.58M |
| 2 | 333.48M | 446.36M | 1.96× | 2.95M | 3.85M | 0.99× | 89.11M |
| 4 | 516.87M | 623.74M | 2.74× | 2.72M | 3.21M | 0.82× | 90.28M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
