# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.75M | 0.008 | 124.95M | 0.033 | 3.36× | 4.17× |
| 10,000 | 0.072 | 138.55M | 0.071 | 140.12M | 0.106 | 1.47× | 1.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.403 | 0.432 | 2.31M | 36.835 | 85.24× | 66.14× |
| 1,500 | 10 | 3.376 | 1.369 | 7.31M | 40.391 | 29.51× | 21.24× |
| 1,500 | 100 | 6.700 | 3.958 | 25.27M | 35.757 | 9.03× | 7.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.75M | 14.85M | 1.00× | 1.22M | 1.03M | 1.00× | 7.98M |
| 2 | 16.02M | 17.83M | 1.20× | 1.11M | 1.26M | 1.22× | 9.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
