# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.93M | 0.027 | 36.65M | 0.038 | 1.19× | 1.41× |
| 10,000 | 0.315 | 31.72M | 0.321 | 31.18M | 0.132 | 0.42× | 0.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.044 ms**; native kernel **0.044 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.407 | 0.325 | 3.08M | 40.798 | 125.70× | 89.72× |
| 1,500 | 10 | 2.895 | 1.473 | 6.79M | 40.359 | 27.41× | 18.80× |
| 1,500 | 100 | 8.369 | 5.707 | 17.52M | 43.314 | 7.59× | 5.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
