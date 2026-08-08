# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.25M | 0.009 | 117.29M | 0.032 | 3.06× | 3.81× |
| 10,000 | 0.076 | 131.70M | 0.074 | 135.20M | 0.086 | 1.13× | 1.16× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.013 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.372 | 0.290 | 3.45M | 31.965 | 110.30× | 93.46× |
| 1,500 | 10 | 2.627 | 1.197 | 8.35M | 33.984 | 28.38× | 23.82× |
| 1,500 | 100 | 6.263 | 3.248 | 30.79M | 35.142 | 10.82× | 8.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
