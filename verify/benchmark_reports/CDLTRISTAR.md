# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.94M | 0.021 | 47.95M | 0.034 | 1.44× | 1.65× |
| 10,000 | 0.201 | 49.83M | 0.189 | 52.87M | 0.090 | 0.45× | 0.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.029 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.370 | 0.287 | 3.48M | 38.617 | 134.45× | 102.81× |
| 1,500 | 10 | 2.671 | 1.288 | 7.76M | 37.407 | 29.04× | 22.57× |
| 1,500 | 100 | 7.259 | 4.502 | 22.21M | 38.663 | 8.59× | 6.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
