# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.55M | 0.008 | 120.72M | 0.038 | 3.99× | 4.61× |
| 10,000 | 0.056 | 178.95M | 0.052 | 192.59M | 0.087 | 1.55× | 1.67× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.206 | 4.85M | 39.979 | 193.80× | 151.47× |
| 1,500 | 10 | 1.705 | 0.882 | 11.34M | 40.281 | 45.68× | 35.73× |
| 1,500 | 100 | 5.626 | 2.375 | 42.10M | 40.304 | 16.97× | 14.16× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.14M | 7.37M | 1.00× | 1.04M | 1.38M | 1.00× | 8.63M |
| 2 | 11.71M | 20.64M | 2.80× | 1.07M | 1.55M | 1.12× | 8.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
