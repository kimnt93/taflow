# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.279 | 3.59M | 0.024 | 41.95M | 0.041 | 0.15× | 1.73× |
| 10,000 | 2.639 | 3.79M | 0.220 | 45.54M | 0.091 | 0.03× | 0.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.401 ms**; native kernel **0.032 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.348 | 0.222 | 4.49M | 44.269 | 198.96× | 157.14× |
| 1,500 | 10 | 5.942 | 1.066 | 9.39M | 41.271 | 38.73× | 34.51× |
| 1,500 | 100 | 31.638 | 4.453 | 22.46M | 42.080 | 9.45× | 11.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
