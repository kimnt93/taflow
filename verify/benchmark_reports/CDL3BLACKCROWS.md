# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.93M | 0.009 | 112.67M | 0.032 | 2.79× | 3.62× |
| 10,000 | 0.119 | 84.03M | 0.119 | 83.96M | 0.090 | 0.76× | 0.76× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.014 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.406 | 0.315 | 3.17M | 37.674 | 119.59× | 97.25× |
| 1,500 | 10 | 2.948 | 2.242 | 4.46M | 34.963 | 15.59× | 13.80× |
| 1,500 | 100 | 7.674 | 27.268 | 3.67M | 35.046 | 1.29× | 1.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
