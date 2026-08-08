# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.52M | 0.026 | 39.18M | 0.043 | 1.56× | 1.67× |
| 10,000 | 0.288 | 34.67M | 0.288 | 34.76M | 0.172 | 0.60× | 0.60× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.039 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.407 | 0.325 | 3.07M | 48.561 | 149.32× | 87.63× |
| 1,500 | 10 | 2.818 | 1.380 | 7.25M | 48.194 | 34.92× | 21.68× |
| 1,500 | 100 | 33.237 | 7.664 | 13.05M | 51.645 | 6.74× | 3.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
