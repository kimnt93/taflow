# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.29M | 0.008 | 131.59M | 0.034 | 3.59× | 4.45× |
| 10,000 | 0.069 | 144.99M | 0.062 | 160.88M | 0.126 | 1.83× | 2.03× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.356 | 0.272 | 3.67M | 36.765 | 135.10× | 105.56× |
| 1,500 | 10 | 2.533 | 1.272 | 7.86M | 38.609 | 30.36× | 23.02× |
| 1,500 | 100 | 5.596 | 3.191 | 31.33M | 37.878 | 11.87× | 9.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.70M | 15.41M | 1.00× | 1.13M | 1.02M | 1.00× | 9.30M |
| 2 | 13.89M | 16.07M | 1.04× | 1.33M | 1.12M | 1.10× | 9.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
