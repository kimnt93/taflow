# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.01M | 0.021 | 47.79M | 0.042 | 1.80× | 2.00× |
| 10,000 | 0.217 | 45.99M | 0.204 | 48.99M | 0.095 | 0.44× | 0.47× |
| 100,000 | 2.127 | 47.01M | 2.071 | 48.28M | 0.586 | 0.28× | 0.28× |
| 1,000,000 | 21.620 | 46.25M | 20.528 | 48.71M | 5.695 | 0.26× | 0.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.115 ms**; native kernel **2.027 ms**; TA-Lib 0.578 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.302 | 0.207 | 4.83M | 574.581 | 2776.65× | 163.24× |
| 100,000 | 10 | 1.780 | 0.920 | 10.87M | 577.345 | 627.75× | 36.73× |
| 100,000 | 1,000 | 23.751 | 21.998 | 45.46M | 575.360 | 26.16× | 1.82× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 43.22M | 44.93M | 1.00× | 2.25M | 3.03M | 1.00× | 132.71M |
| 2 | 80.71M | 87.09M | 1.94× | 2.26M | 2.79M | 0.92× | 132.49M |
| 4 | 142.96M | 167.74M | 3.73× | 2.28M | 2.55M | 0.84× | 134.31M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
