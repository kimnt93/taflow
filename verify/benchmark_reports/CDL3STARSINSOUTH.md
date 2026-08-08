# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.86M | 0.011 | 94.14M | 0.033 | 2.56× | 3.09× |
| 10,000 | 0.124 | 80.75M | 0.119 | 84.00M | 0.117 | 0.94× | 0.98× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.016 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.533 | 0.445 | 2.25M | 35.456 | 79.65× | 62.67× |
| 1,500 | 10 | 2.708 | 1.303 | 7.67M | 38.092 | 29.23× | 21.76× |
| 1,500 | 100 | 6.584 | 3.865 | 25.87M | 39.400 | 10.19× | 7.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
