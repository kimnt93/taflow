# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.08M | 0.015 | 66.31M | 0.036 | 1.78× | 2.36× |
| 10,000 | 0.194 | 51.63M | 0.190 | 52.51M | 0.199 | 1.03× | 1.04× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.024 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.418 | 0.290 | 3.45M | 40.098 | 138.30× | 98.13× |
| 1,500 | 10 | 2.665 | 1.261 | 7.93M | 39.774 | 31.54× | 23.08× |
| 1,500 | 100 | 6.911 | 4.277 | 23.38M | 42.999 | 10.05× | 6.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
