# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.92M | 0.005 | 208.32M | 0.035 | 5.35× | 7.39× |
| 10,000 | 0.068 | 147.83M | 0.064 | 155.91M | 0.113 | 1.68× | 1.77× |
| 100,000 | 0.728 | 137.41M | 0.720 | 138.87M | 0.895 | 1.23× | 1.24× |
| 1,000,000 | 7.649 | 130.73M | 7.545 | 132.54M | 8.855 | 1.16× | 1.17× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.723 ms**; native kernel **0.710 ms**; TA-Lib 0.906 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.263 | 3.81M | 900.481 | 3426.63× | 108.26× |
| 100,000 | 10 | 2.630 | 1.372 | 7.29M | 879.659 | 641.20× | 20.44× |
| 100,000 | 1,000 | 29.050 | 26.082 | 38.34M | 888.476 | 34.07× | 1.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 107.44M | 106.53M | 1.00× | 2.48M | 2.07M | 1.00× | 93.04M |
| 2 | 217.11M | 225.23M | 2.11× | 2.25M | 2.70M | 1.31× | 96.61M |
| 4 | 378.31M | 406.46M | 3.82× | 2.28M | 2.46M | 1.19× | 96.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
