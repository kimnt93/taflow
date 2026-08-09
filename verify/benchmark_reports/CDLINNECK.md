# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.07M | 0.008 | 129.92M | 0.035 | 3.67× | 4.53× |
| 10,000 | 0.070 | 142.26M | 0.068 | 146.97M | 0.123 | 1.74× | 1.80× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.357 | 0.282 | 3.55M | 36.408 | 129.09× | 100.42× |
| 1,500 | 10 | 2.573 | 1.285 | 7.78M | 37.184 | 28.93× | 22.16× |
| 1,500 | 100 | 5.952 | 3.630 | 27.55M | 38.243 | 10.54× | 7.96× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.86M | 8.42M | 1.00× | 1.20M | 1.36M | 1.00× | 9.40M |
| 2 | 18.12M | 20.14M | 2.39× | 1.27M | 1.04M | 0.77× | 9.57M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
