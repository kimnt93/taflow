# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.16M | 0.013 | 74.75M | 0.036 | 2.35× | 2.66× |
| 10,000 | 0.146 | 68.28M | 0.148 | 67.52M | 0.134 | 0.91× | 0.90× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.020 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.387 | 0.294 | 3.40M | 41.538 | 141.16× | 97.19× |
| 1,500 | 10 | 2.710 | 3.752 | 2.67M | 40.797 | 10.87× | 11.92× |
| 1,500 | 100 | 11.018 | 6.723 | 14.87M | 41.291 | 6.14× | 4.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
