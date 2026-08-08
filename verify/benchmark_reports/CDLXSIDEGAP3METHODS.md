# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.01M | 0.011 | 94.48M | 0.031 | 2.44× | 2.95× |
| 10,000 | 0.115 | 86.88M | 0.113 | 88.29M | 0.086 | 0.74× | 0.76× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.015 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.370 | 0.289 | 3.46M | 34.244 | 118.34× | 100.96× |
| 1,500 | 10 | 4.346 | 1.209 | 8.27M | 34.527 | 28.56× | 24.62× |
| 1,500 | 100 | 6.335 | 3.683 | 27.15M | 34.784 | 9.44× | 8.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
