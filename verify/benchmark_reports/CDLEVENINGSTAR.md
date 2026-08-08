# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.30M | 0.005 | 212.30M | 0.037 | 5.34× | 7.91× |
| 10,000 | 0.081 | 123.43M | 0.075 | 133.33M | 0.115 | 1.41× | 1.53× |
| 100,000 | 0.871 | 114.82M | 0.862 | 115.98M | 0.860 | 0.99× | 1.00× |
| 1,000,000 | 9.106 | 109.82M | 8.985 | 111.30M | 8.656 | 0.95× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.057 ms**; native kernel **1.011 ms**; TA-Lib 0.925 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.420 | 0.342 | 2.93M | 890.550 | 2606.60× | 91.24× |
| 100,000 | 10 | 2.699 | 1.624 | 6.16M | 863.285 | 531.69× | 19.89× |
| 100,000 | 1,000 | 33.387 | 32.240 | 31.02M | 1143.701 | 35.47× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.37M | 95.93M | 1.00× | 1.46M | 2.39M | 1.00× | 92.18M |
| 2 | 169.16M | 196.62M | 2.05× | 2.16M | 2.66M | 1.12× | 94.84M |
| 4 | 321.03M | 329.04M | 3.43× | 2.23M | 2.47M | 1.04× | 97.88M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
