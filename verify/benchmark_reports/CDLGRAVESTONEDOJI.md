# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.61M | 0.013 | 78.55M | 0.037 | 3.65× | 2.94× |
| 10,000 | 0.057 | 174.23M | 0.053 | 187.22M | 0.102 | 1.78× | 1.91× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.712 | 0.555 | 1.80M | 39.108 | 70.46× | 64.90× |
| 1,500 | 10 | 2.552 | 1.252 | 7.99M | 37.490 | 29.95× | 22.64× |
| 1,500 | 100 | 5.005 | 2.764 | 36.18M | 38.169 | 13.81× | 10.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.06M | 9.51M | 1.00× | 1.21M | 765.85K | 1.00× | 7.82M |
| 2 | 16.44M | 16.86M | 1.77× | 1.32M | 1.42M | 1.85× | 9.13M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
