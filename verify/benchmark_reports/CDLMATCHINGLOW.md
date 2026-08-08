# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.36M | 0.004 | 272.56M | 0.032 | 5.71× | 8.73× |
| 10,000 | 0.045 | 223.25M | 0.042 | 237.32M | 0.091 | 2.03× | 2.16× |
| 100,000 | 0.493 | 202.67M | 0.473 | 211.47M | 0.668 | 1.35× | 1.41× |
| 1,000,000 | 5.598 | 178.62M | 5.360 | 186.56M | 6.890 | 1.23× | 1.29× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.511 ms**; native kernel **0.498 ms**; TA-Lib 0.676 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.289 | 3.46M | 665.763 | 2306.64× | 97.98× |
| 100,000 | 10 | 2.686 | 1.327 | 7.54M | 653.098 | 492.21× | 22.65× |
| 100,000 | 1,000 | 26.790 | 22.681 | 44.09M | 678.598 | 29.92× | 1.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 148.40M | 157.40M | 1.00× | 2.27M | 2.55M | 1.00× | 106.99M |
| 2 | 286.22M | 319.60M | 2.03× | 2.19M | 2.49M | 0.98× | 131.34M |
| 4 | 509.18M | 573.59M | 3.64× | 2.43M | 2.52M | 0.99× | 128.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
