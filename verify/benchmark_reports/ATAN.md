# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.36M | 0.008 | 130.96M | 0.034 | 0.69× | 4.46× |
| 10,000 | 0.483 | 20.70M | 0.068 | 146.58M | 0.089 | 0.18× | 1.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.072 ms**; native kernel **0.011 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.295 | 0.177 | 5.65M | 38.516 | 217.78× | 146.74× |
| 1,500 | 10 | 1.695 | 0.699 | 14.31M | 35.872 | 51.35× | 37.10× |
| 1,500 | 100 | 6.623 | 2.679 | 37.33M | 38.243 | 14.27× | 9.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
