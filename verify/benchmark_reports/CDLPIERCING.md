# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.68M | 0.008 | 131.54M | 0.033 | 3.41× | 4.29× |
| 10,000 | 0.083 | 120.27M | 0.077 | 130.27M | 0.121 | 1.46× | 1.58× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.352 | 0.276 | 3.63M | 34.829 | 126.35× | 111.04× |
| 1,500 | 10 | 2.623 | 1.274 | 7.85M | 37.349 | 29.31× | 22.96× |
| 1,500 | 100 | 5.588 | 3.074 | 32.53M | 36.594 | 11.90× | 9.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.94M | 14.22M | 1.00× | 1.03M | 1.04M | 1.00× | 7.75M |
| 2 | 15.92M | 14.20M | 1.00× | 1.24M | 1.40M | 1.34× | 9.32M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
