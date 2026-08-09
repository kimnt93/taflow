# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.55M | 0.004 | 256.59M | 0.031 | 5.48× | 7.96× |
| 10,000 | 0.052 | 191.40M | 0.047 | 211.08M | 0.095 | 1.81× | 2.00× |
| 100,000 | 0.641 | 156.01M | 0.617 | 161.95M | 0.744 | 1.16× | 1.20× |
| 1,000,000 | 6.652 | 150.33M | 6.484 | 154.22M | 7.204 | 1.08× | 1.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.636 ms**; native kernel **0.620 ms**; TA-Lib 0.740 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.337 | 0.260 | 3.84M | 748.265 | 2876.63× | 106.93× |
| 100,000 | 10 | 2.573 | 1.390 | 7.20M | 769.957 | 554.04× | 21.09× |
| 100,000 | 1,000 | 28.428 | 25.247 | 39.61M | 741.508 | 29.37× | 1.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 125.23M | 127.36M | 1.00× | 2.71M | 2.96M | 1.00× | 118.98M |
| 2 | 241.60M | 248.25M | 1.95× | 2.42M | 2.69M | 0.91× | 110.76M |
| 4 | 420.10M | 458.64M | 3.60× | 2.25M | 2.42M | 0.82× | 108.37M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
