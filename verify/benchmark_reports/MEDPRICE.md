# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.91M | 0.003 | 301.08M | 0.029 | 6.39× | 8.78× |
| 10,000 | 0.010 | 1.02G | 0.007 | 1.44G | 0.033 | 3.32× | 4.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.003 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.318 | 0.196 | 5.10M | 28.821 | 147.01× | 139.59× |
| 1,500 | 10 | 1.610 | 0.783 | 12.77M | 29.257 | 37.37× | 35.86× |
| 1,500 | 100 | 3.313 | 1.851 | 54.03M | 28.996 | 15.67× | 17.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.17M | 13.49M | 1.00× | 1.08M | 1.52M | 1.00× | 10.04M |
| 2 | 21.75M | 18.41M | 1.36× | 1.38M | 1.63M | 1.07× | 10.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
