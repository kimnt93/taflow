# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.14M | 0.007 | 143.58M | 0.031 | 3.58× | 4.43× |
| 10,000 | 0.103 | 97.29M | 0.096 | 104.16M | 0.085 | 0.83× | 0.88× |
| 100,000 | 1.191 | 83.95M | 1.160 | 86.22M | 0.571 | 0.48× | 0.49× |
| 1,000,000 | 12.151 | 82.30M | 11.996 | 83.36M | 5.727 | 0.47× | 0.48× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.190 ms**; native kernel **1.166 ms**; TA-Lib 0.567 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.278 | 3.59M | 560.019 | 2011.84× | 97.99× |
| 100,000 | 10 | 2.547 | 1.382 | 7.24M | 571.756 | 413.71× | 20.38× |
| 100,000 | 1,000 | 29.610 | 25.857 | 38.67M | 585.314 | 22.64× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.89M | 75.03M | 1.00× | 2.26M | 2.48M | 1.00× | 133.22M |
| 2 | 134.48M | 140.92M | 1.88× | 2.29M | 2.55M | 1.03× | 132.16M |
| 4 | 251.33M | 267.00M | 3.56× | 2.28M | 2.43M | 0.98× | 132.52M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
