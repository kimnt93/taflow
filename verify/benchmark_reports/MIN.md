# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.58M | 0.005 | 208.92M | 0.034 | 5.95× | 7.16× |
| 10,000 | 0.035 | 287.00M | 0.033 | 306.67M | 0.077 | 2.20× | 2.35× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.297 | 0.175 | 5.72M | 36.301 | 207.49× | 173.70× |
| 1,500 | 10 | 1.169 | 1.128 | 8.86M | 38.175 | 33.84× | 29.51× |
| 1,500 | 100 | 4.000 | 2.413 | 41.45M | 37.476 | 15.53× | 13.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.94M | 16.92M | 1.00× | 1.24M | 1.34M | 1.00× | 9.24M |
| 2 | 18.28M | 17.32M | 1.02× | 1.43M | 1.37M | 1.02× | 9.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
