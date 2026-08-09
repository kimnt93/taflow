# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.25M | 0.008 | 128.06M | 0.036 | 4.00× | 4.61× |
| 10,000 | 0.057 | 174.00M | 0.056 | 178.80M | 0.079 | 1.37× | 1.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.270 | 0.203 | 4.92M | 40.128 | 197.63× | 151.49× |
| 1,500 | 10 | 1.526 | 0.787 | 12.71M | 38.932 | 49.50× | 39.29× |
| 1,500 | 100 | 3.606 | 2.180 | 45.87M | 38.478 | 17.65× | 14.45× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.96M | 14.30M | 1.00× | 1.31M | 1.48M | 1.00× | 8.13M |
| 2 | 17.89M | 19.86M | 1.39× | 1.56M | 1.54M | 1.04× | 9.33M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
