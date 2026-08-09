# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.79M | 0.009 | 107.25M | 0.039 | 3.32× | 4.20× |
| 10,000 | 0.097 | 103.47M | 0.105 | 95.62M | 0.117 | 1.21× | 1.12× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.351 | 0.276 | 3.63M | 44.456 | 161.32× | 123.67× |
| 1,500 | 10 | 2.691 | 1.350 | 7.41M | 43.302 | 32.06× | 25.31× |
| 1,500 | 100 | 6.261 | 3.655 | 27.36M | 45.035 | 12.32× | 9.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.88M | 14.21M | 1.00× | 1.19M | 942.09K | 1.00× | 7.76M |
| 2 | 15.61M | 19.78M | 1.39× | 1.37M | 1.48M | 1.57× | 7.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
