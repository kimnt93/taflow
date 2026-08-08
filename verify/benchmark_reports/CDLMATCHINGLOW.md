# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.86M | 0.013 | 76.89M | 0.032 | 2.12× | 2.48× |
| 10,000 | 0.134 | 74.55M | 0.131 | 76.30M | 0.090 | 0.67× | 0.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.021 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.385 | 0.293 | 3.42M | 34.223 | 116.98× | 96.26× |
| 1,500 | 10 | 2.639 | 1.226 | 8.16M | 35.373 | 28.85× | 24.22× |
| 1,500 | 100 | 29.448 | 3.905 | 25.61M | 37.613 | 9.63× | 7.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
