# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.25M | 0.005 | 218.22M | 0.038 | 6.40× | 8.20× |
| 10,000 | 0.034 | 294.16M | 0.042 | 238.55M | 0.099 | 2.93× | 2.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.007 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.310 | 0.198 | 5.04M | 38.970 | 196.43× | 151.66× |
| 1,500 | 10 | 1.223 | 0.652 | 15.34M | 36.766 | 56.41× | 45.91× |
| 1,500 | 100 | 3.952 | 2.428 | 41.18M | 38.703 | 15.94× | 13.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.30M | 14.25M | 1.00× | 1.29M | 1.57M | 1.00× | 8.23M |
| 2 | 16.72M | 16.57M | 1.16× | 1.47M | 1.26M | 0.80× | 8.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
