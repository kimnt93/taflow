# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.46M | 0.025 | 39.68M | 0.032 | 1.18× | 1.28× |
| 10,000 | 0.272 | 36.71M | 0.264 | 37.94M | 0.096 | 0.35× | 0.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.038 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.393 | 0.305 | 3.27M | 34.186 | 111.92× | 93.42× |
| 1,500 | 10 | 2.825 | 1.789 | 5.59M | 33.262 | 18.59× | 16.36× |
| 1,500 | 100 | 8.274 | 5.568 | 17.96M | 35.744 | 6.42× | 5.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
