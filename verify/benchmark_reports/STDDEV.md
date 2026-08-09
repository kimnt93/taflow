# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.26M | 0.006 | 170.94M | 0.037 | 5.21× | 6.25× |
| 10,000 | 0.041 | 241.84M | 0.041 | 241.33M | 0.072 | 1.74× | 1.74× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.299 | 0.169 | 5.92M | 38.776 | 229.70× | 191.32× |
| 1,500 | 10 | 1.152 | 0.637 | 15.70M | 37.775 | 59.30× | 50.59× |
| 1,500 | 100 | 3.005 | 2.068 | 48.36M | 36.082 | 17.45× | 16.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.79M | 9.48M | 1.00× | 1.08M | 1.50M | 1.00× | 8.28M |
| 2 | 19.52M | 22.52M | 2.38× | 1.50M | 1.76M | 1.17× | 9.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
