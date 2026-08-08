# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.83M | 0.005 | 213.29M | 0.034 | 5.05× | 7.18× |
| 10,000 | 0.090 | 111.37M | 0.085 | 117.33M | 0.132 | 1.47× | 1.55× |
| 100,000 | 0.985 | 101.49M | 0.957 | 104.54M | 1.032 | 1.05× | 1.08× |
| 1,000,000 | 10.176 | 98.27M | 9.912 | 100.89M | 10.217 | 1.00× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.963 ms**; native kernel **0.948 ms**; TA-Lib 1.031 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.372 | 0.298 | 3.35M | 1061.618 | 3557.03× | 88.57× |
| 100,000 | 10 | 2.767 | 1.461 | 6.84M | 1046.853 | 716.42× | 18.85× |
| 100,000 | 1,000 | 30.678 | 27.784 | 35.99M | 1065.705 | 38.36× | 1.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.78M | 91.89M | 1.00× | 1.84M | 2.31M | 1.00× | 83.61M |
| 2 | 178.36M | 171.15M | 1.86× | 2.29M | 2.45M | 1.06× | 80.19M |
| 4 | 304.71M | 328.77M | 3.58× | 2.26M | 2.43M | 1.05× | 85.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
