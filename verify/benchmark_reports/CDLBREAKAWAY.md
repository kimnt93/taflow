# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.51M | 0.026 | 38.85M | 0.032 | 1.11× | 1.25× |
| 10,000 | 0.311 | 32.14M | 0.319 | 31.38M | 0.097 | 0.31× | 0.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.042 ms**; native kernel **0.040 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.400 | 0.314 | 3.18M | 33.897 | 107.88× | 92.24× |
| 1,500 | 10 | 2.934 | 1.537 | 6.51M | 34.601 | 22.51× | 18.59× |
| 1,500 | 100 | 8.960 | 5.784 | 17.29M | 33.351 | 5.77× | 4.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
