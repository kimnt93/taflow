# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.27M | 0.008 | 118.61M | 0.041 | 4.03× | 4.82× |
| 10,000 | 0.124 | 80.94M | 0.124 | 80.57M | 0.183 | 1.48× | 1.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.365 | 0.280 | 3.57M | 48.857 | 174.46× | 110.54× |
| 1,500 | 10 | 2.617 | 1.323 | 7.56M | 60.036 | 45.36× | 21.85× |
| 1,500 | 100 | 6.081 | 3.730 | 26.81M | 50.233 | 13.47× | 7.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.97M | 12.35M | 1.00× | 982.42K | 1.24M | 1.00× | 8.49M |
| 2 | 14.28M | 19.37M | 1.57× | 1.29M | 1.31M | 1.06× | 9.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
