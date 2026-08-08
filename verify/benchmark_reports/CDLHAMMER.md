# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.23M | 0.014 | 69.21M | 0.042 | 2.63× | 2.92× |
| 10,000 | 0.128 | 78.39M | 0.125 | 80.11M | 0.176 | 1.38× | 1.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.020 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.360 | 0.286 | 3.49M | 52.518 | 183.31× | 104.07× |
| 1,500 | 10 | 2.679 | 1.236 | 8.09M | 49.257 | 39.85× | 24.18× |
| 1,500 | 100 | 6.523 | 4.073 | 24.55M | 49.794 | 12.23× | 7.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
