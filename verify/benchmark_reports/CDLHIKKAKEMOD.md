# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.05M | 0.010 | 98.40M | 0.033 | 2.69× | 3.27× |
| 10,000 | 0.115 | 86.97M | 0.110 | 90.96M | 0.084 | 0.73× | 0.76× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.015 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.365 | 0.276 | 3.63M | 35.334 | 128.16× | 102.35× |
| 1,500 | 10 | 2.593 | 1.174 | 8.52M | 37.865 | 32.25× | 25.30× |
| 1,500 | 100 | 6.336 | 3.465 | 28.86M | 37.130 | 10.72× | 8.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
