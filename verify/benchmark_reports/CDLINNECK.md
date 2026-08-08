# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.50M | 0.023 | 42.79M | 0.034 | 1.31× | 1.46× |
| 10,000 | 0.268 | 37.38M | 0.266 | 37.58M | 0.127 | 0.47× | 0.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.037 ms**; native kernel **0.036 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.309 | 3.23M | 37.972 | 122.73× | 91.36× |
| 1,500 | 10 | 2.767 | 1.362 | 7.34M | 39.874 | 29.28× | 21.31× |
| 1,500 | 100 | 8.341 | 8.338 | 11.99M | 39.868 | 4.78× | 3.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
