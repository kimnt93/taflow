# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.80M | 0.018 | 55.39M | 0.048 | 2.54× | 2.66× |
| 10,000 | 0.169 | 59.18M | 0.159 | 62.89M | 0.227 | 1.35× | 1.43× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.023 ms**; TA-Lib 0.058 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.713 | 0.281 | 3.56M | 56.134 | 200.08× | 99.02× |
| 1,500 | 10 | 2.727 | 1.297 | 7.71M | 55.517 | 42.82× | 22.56× |
| 1,500 | 100 | 6.740 | 3.996 | 25.03M | 58.293 | 14.59× | 7.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
