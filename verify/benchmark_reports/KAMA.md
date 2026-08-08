# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.48M | 0.012 | 82.45M | 0.034 | 0.62× | 2.77× |
| 10,000 | 0.496 | 20.18M | 0.110 | 91.03M | 0.063 | 0.13× | 0.58× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.080 ms**; native kernel **0.017 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.455 | 0.302 | 3.31M | 35.788 | 118.31× | 103.71× |
| 1,500 | 10 | 1.856 | 0.835 | 11.98M | 37.281 | 44.65× | 36.65× |
| 1,500 | 100 | 8.170 | 3.331 | 30.02M | 35.718 | 10.72× | 9.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
